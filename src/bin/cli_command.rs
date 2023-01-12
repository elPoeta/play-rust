#![allow(dead_code)]

use std::{fmt, process::Command};

#[derive(Debug)]
enum CMD {
    Cat,
    Ls,
}

impl fmt::Display for CMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
fn main() {
    let cat = &CMD::Cat.to_string().to_lowercase();
    let mut cmd = Command::new(cat);
    cmd.arg("Cargo.toml");

    match cmd.output() {
        Ok(o) => unsafe { println!("{}", String::from_utf8_unchecked(o.stdout)) },
        Err(e) => println!("There was an error {}", e),
    }

    let out = Command::new(cat)
        .arg("README.md")
        .output()
        .expect("failed to execute process");
    unsafe { println!("{}", String::from_utf8_unchecked(out.stdout)) }

    let ls = CMD::Ls.to_string().to_lowercase();
    let mut list_dir = Command::new(ls);

    list_dir.status().expect("process failed to execute");

    println!();

    list_dir.current_dir("/");

    list_dir.status().expect("process failed to execute");
}
