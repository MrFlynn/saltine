use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn get_user_info(path: &str, username: &str) -> (String, String) {
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