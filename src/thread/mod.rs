use std::thread;

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{mpsc, Arc};

use std::time::Instant;

use crate::crypt;
use crate::generator::WordGenerator;

struct Solution(String);

fn find_password(
    hash: String,
    salt: String,
    base_chars: Vec<char>,
    size: usize,
    alphabet: Vec<char>,
    sender: mpsc::Sender<Solution>,
    terminate: Arc<AtomicBool>,
    counter: Arc<AtomicIsize>,
) {
    for c in base_chars {
        let word_space = alphabet.clone();
        let mut generator = WordGenerator::new(c, size, word_space);

        let mut iterations: isize = 0;
        loop {
            match generator.next() {
                Some(s) => {
                    if hash == crypt::md5(&s, &salt) {
                        terminate.store(true, Ordering::Relaxed);
                        match sender.send(Solution(s)) {
                            Ok(_) => break,
                            Err(_) => panic!("Receiver stopped listening!"),
                        }
                    }

                    iterations += 1;
                    if (iterations % 10000) == 0 {
                        let prev = counter.fetch_add(iterations, Ordering::Relaxed);
                        println!("Tried {} passwords", prev + iterations);
                    }
                }
                None => break,
            }
        }
    }
}

fn create_buckets(num_threads: u32, alphabet: Vec<char>) -> Vec<Vec<char>> {
    let mut buckets = vec![Vec::new(); num_threads as usize];

    for (i, c) in alphabet.iter().enumerate() {
        buckets[i % (num_threads as usize)].push(*c);
    }

    buckets
}

pub fn run(hash: String, salt: String, size: usize, threads: u32, alphabet: &str) {
    let word_space: Vec<char> = alphabet.chars().collect();
    let buckets = create_buckets(threads, word_space.clone());

    // Thread communication.
    let terminate = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();

    // Global counter.
    let counter = Arc::new(AtomicIsize::new(0));

    // Program timer.
    let timer = Instant::now();

    for t in 0..threads {
        // Copy the specific bucket for the thread into the current context.
        let bucket = buckets[t as usize].clone();

        // Cloned value to be used in new OS thread.
        let thread_hash = hash.clone();
        let thread_salt = salt.clone();
        let thread_size = size.clone();
        let thread_word_space = word_space.clone();
        let thread_sender = sender.clone();
        let thread_terminate = terminate.clone();
        let thread_counter = counter.clone();

        thread::spawn(move || {
            find_password(
                thread_hash,
                thread_salt,
                bucket,
                thread_size,
                thread_word_space,
                thread_sender,
                thread_terminate,
                thread_counter,
            );
        });
    }

    match receiver.recv() {
        Ok(status) => {
            println!("Tried {} passwords", counter.load(Ordering::Relaxed));
            println!("Password found: {}", status.0);
            println!("Program completed in {:?}s", timer.elapsed());
        }
        Err(_) => panic!("Worker threads disconnected!"),
    }
}
