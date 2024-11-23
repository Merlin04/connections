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

pub fn release_batch() {
    let batch_out_dir: &str = &*(dotenv::var(ZOLA_ROOT_VAR).unwrap() + "/content");
    let mut existing_batches = fs::read_dir(batch_out_dir).unwrap();
    let proposed_batch_name = Local::now().format("%a %b %e, %Y").to_string();
    let batch_name = if existing_batches.any(|e| { e.unwrap().path().file_name().unwrap().to_str().unwrap() == proposed_batch_name }) {
        Local::now().format("%a %b %e %T, %Y").to_string()
    } else { proposed_batch_name };
    // move over the files
    fs::create_dir(batch_name).unwrap();
    fs::read_dir(dotenv::var(WIP_DIR_VAR).unwrap()).unwrap().for_each(|e| {
        let path = e.unwrap().path();
        fs::rename(path.to_str().unwrap(), batch_out_dir.to_owned() + "/" + path.file_name().unwrap().to_str().unwrap()).unwrap();
    });

    rebuild_zola();
}