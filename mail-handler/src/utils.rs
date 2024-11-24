use std::{fs, io};
use std::io::Write;
use std::process::Command;
use crate::consts::{WIP_DIR_VAR, WWW_OUT_DIR_VAR, ZOLA_ROOT_VAR};
use chrono::prelude::*;

pub fn rebuild_zola() {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "zola -r {} build -o {} --force",
            dotenv::var(ZOLA_ROOT_VAR).unwrap(),
            dotenv::var(WWW_OUT_DIR_VAR).unwrap()
        ))
        .output()
        .expect("failed to execute zola build");
    io::stdout().write_all(&output. stdout).unwrap();
    io::stderr().write_all(&output. stderr).unwrap();
    println!("Wrote zola output to {}", dotenv::var(WWW_OUT_DIR_VAR).unwrap());
}

pub fn batch_index_contents(
    display_name: &str
) -> String {
    format!(
        "+++
title = \"{display_name}\"
sort_by = \"title\"
paginate_reversed = true
template = \"connections.html\"
page_template = \"connection.html\"
paginate_by = 50
weight = {}
+++"
    , Local::now().timestamp())
}

pub fn release_batch() {
    let batch_out_dir: &str = &*(dotenv::var(ZOLA_ROOT_VAR).unwrap() + "/content");
    let mut existing_batches = fs::read_dir(batch_out_dir).unwrap();
    let proposed_slug = Local::now().format("%Y-%m-%d").to_string();
    let (slug, display_name) = if existing_batches.any(|e| { e.unwrap().path().file_name().unwrap().to_str().unwrap() == proposed_slug }) {
        (Local::now().format("%Y-%m-%d-%s").to_string(), Local::now().format("%a %b %e %T, %Y").to_string())
    } else { (proposed_slug, Local::now().format("%a %b %e, %Y").to_string()) };
    let batch_path = batch_out_dir.to_owned() + "/" + &*slug;
    println!("Attempting to write batch to {}", batch_path);
    // move over the files
    fs::create_dir(batch_path.to_owned()).unwrap();
    fs::read_dir(dotenv::var(WIP_DIR_VAR).unwrap()).unwrap().for_each(|e| {
        let path = e.unwrap().path();
        let from = path.to_str().unwrap();
        let to = batch_path.to_owned() + "/" + path.file_name().unwrap().to_str().unwrap();
        println!("moving {from} to {to}");
        // fs::rename(from, to).unwrap();
        // cross device link fix
        fs::copy(from, to).unwrap();
        fs::remove_file(from).unwrap();
    });
    fs::write(batch_path + "/_index.md", batch_index_contents(&*display_name)).unwrap();

    rebuild_zola();
}

pub fn post_file_contents(
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
<b>{}: </b>{}",
        number,
        date_fmt,
        if is_not_anon { from_addr } else { "anonymous" },
        number,
        html
    )
}