use crate::generator::WordGenerator;
use crate::crypt;

pub fn run(hash: String, salt: String, size: usize, alphabet: &str) {
    let word_space = alphabet.chars().collect();
    let mut generator = WordGenerator::new('a', size, word_space);

    let mut count = 0;
    loop {
        match generator.next() {
            Some(s) => {
                if hash == crypt::md5(&s, &salt) {
                    println!("Password found: {}", s);
                    break;
                }

                count += 1;
                if (count % 10000) == 0 {
                    println!("Tried {} passwords", count);
                }
            }
            None => {
                break;
            }
        }
    }
}