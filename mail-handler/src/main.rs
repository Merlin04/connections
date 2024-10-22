use imap::types::{AttributeValue, UnsolicitedResponse};
use mail_parser::MessageParser;
use std::fs;

fn post_file_contents(
    number: u32,
    date: &mail_parser::DateTime,
    from_addr: &str,
    is_not_anon: bool,
    html: &str,
) -> String {
    let date_fmt = format!("{}-{}-{}", date.year, date.month, date.day);
    format!(
        "+++
title = \"{}\"
date = \"{}\"
authors = [\"{}\"]
+++
{}",
        number,
        date_fmt,
        if is_not_anon { from_addr } else { "anonymous" },
        html
    )
}

fn main() -> imap::error::Result<()> {
    dotenv::dotenv().ok();

    let number_file = dotenv::var("NUMBER_FILE").unwrap();
    let mut use_number = {
        let mut current_number = fs::read_to_string(&number_file)?
            .replace("\n", "")
            .parse::<u32>()
            .unwrap();
        move || {
            let old = current_number;
            current_number += 1;
            fs::write(&number_file, current_number.to_string()).unwrap();
            old
        }
    };

    let make_session = || -> Result<imap::Session<imap::Connection>, imap::error::Error> {
        let client = imap::ClientBuilder::new(
            dotenv::var("IMAP_ADDR").unwrap(),
            dotenv::var("IMAP_PORT").unwrap().parse::<u16>().unwrap(),
        )
        .connect()?;
        let session = client
            .login(
                dotenv::var("IMAP_USERNAME").unwrap(),
                dotenv::var("IMAP_PASSWORD").unwrap(),
            )
            .map_err(|e| e.0)?;
        Ok(session)
    };

    let mut imap_session = make_session()?;

    imap_session.select(dotenv::var("IMAP_MAILBOX_NAME").unwrap())?;

    let mut inner_session = make_session()?;
    imap_session.idle().wait_while(|response| {
        match response {
            UnsolicitedResponse::Fetch { id, attributes } => {
                let flags = attributes.iter().find_map(|v| {
                    if let AttributeValue::Flags(f) = v {
                        Some(f)
                    } else {
                        None
                    }
                });
                if let Some(flags) = flags {
                    if flags.iter().any(|v| v == "\\Flagged") {
                        println!("fetching {}", id.to_string());
                        inner_session
                            .select(dotenv::var("IMAP_MAILBOX_NAME").unwrap())
                            .unwrap();
                        let fetches = inner_session
                            .fetch(id.to_string(), "(BODY.PEEK[])")
                            .unwrap();
                        let message = if let Some(m) = fetches.iter().next() {
                            Ok(m)
                        } else {
                            Err("failed to fetch message after getting notified it exists???")
                        }
                        .unwrap();

                        let message = MessageParser::default()
                            .parse(message.body().unwrap())
                            .unwrap();

                        let from_addr = message
                            .from()
                            .unwrap()
                            .first()
                            .unwrap()
                            .clone()
                            .address
                            .unwrap();
                        let is_not_anon = message
                            .to()
                            .unwrap()
                            .first()
                            .unwrap()
                            .clone()
                            .address
                            .unwrap()
                            == dotenv::var("NON_ANONYMOUS_ADDRESS").unwrap();
                        let html = ammonia::clean(message.body_html(0).unwrap().as_ref());
                        let date = message.date().unwrap();
                        let number = use_number();
                        let contents = post_file_contents(
                            number,
                            date,
                            from_addr.as_ref(),
                            is_not_anon,
                            html.as_ref(),
                        );
                        fs::write(
                            format!("{}/{}.md", dotenv::var("OUT_DIR").unwrap(), number),
                            contents,
                        )
                        .unwrap();
                    }
                }
            }
            _ => (),
        };
        true
    })?;

    imap_session.logout()?;

    Ok(())
}
