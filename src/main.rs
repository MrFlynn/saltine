#[macro_use]
extern crate clap;

mod crypt;
mod thread;

use std::fs::File;
use std::io::{prelude::*, BufReader};

static DEFAULT_ALPHABET: &'static str = &"abcdefghijklmnopqrstuvwxyz";

fn get_user_info(path: &str, username: &str) -> (String, String) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut hash = String::from("");
    let mut salt = String::from("");

    for line in reader.lines() {
        let content = String::from(line.unwrap());

        let first_split: Vec<&str> = content.split_terminator(':').collect();
        if first_split[0] == username {
            let second_split: Vec<&str> = first_split[1].split_terminator('$').collect();

            hash = String::from(second_split[3]);
            salt = String::from(second_split[2]);

            break;
        }
    }

    (hash, salt)
}

fn main() {
    let matches = clap_app!(saltine =>
        (version: "0.1.0")
        (author: "Nick Pleatsikas <nick@pleatsikas.me>")
        (about: "Proof-of-concept MD5 password cracking tool.")
        (@arg FILE: +required "/etc/shadow file to crack.")
        (@arg USERNAME: --username -u +takes_value +required "Username to target.")
        (@arg THREADS: --threads -t +takes_value "Number of threads to run on. Defaults to 1.")
        (@arg SIZE: --("password-size") -s +takes_value "Size of password to crack. Defaults to 6 characters.")
        (@arg ALPHABET: --alphabet -a +takes_value "Character alphabet to use. Defaults to all lower case English characters.")
    ).get_matches();

    let filename: &str = matches.value_of("FILE").unwrap();
    let username: &str = matches.value_of("USERNAME").unwrap();
    let threads: u32 = matches
        .value_of("THREADS")
        .unwrap_or("1")
        .to_string()
        .parse::<u32>()
        .unwrap();
    let size: usize = matches
        .value_of("SIZE")
        .unwrap_or("6")
        .to_string()
        .parse::<usize>()
        .unwrap();
    let alphabet: &str = matches.value_of("ALPHABET").unwrap_or(DEFAULT_ALPHABET);

    let (hash, salt) = get_user_info(filename, username);
}
