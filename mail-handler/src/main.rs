#[macro_use] extern crate rocket;
mod mail;
mod web;
mod utils;
mod consts;

use crate::mail::mail_loop;
use crate::utils::rebuild_zola;
use crate::web::rocket;
use rocket::tokio::runtime::Runtime;
use std::thread;
use crate::consts::REDIS_URI;

#[rocket::main]
async fn main() -> imap::error::Result<()> {
    dotenv::dotenv().ok();

    rebuild_zola();

    println!("Connecting to redis...");
    let client = redis::Client::open(REDIS_URI).unwrap();
    let con = client.get_connection().unwrap();
    println!("Starting web server...");
    // https://stackoverflow.com/a/78657106
    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            rocket().await
        })
    });

    println!("Up!");
    mail_loop(con)
}
