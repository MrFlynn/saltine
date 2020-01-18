# Saltine

Saltine is a multithreaded brute-force password cracking utility written in Rust.
This program was build for the computer security class at UC Riverside (CS 165).
As such, this program should only be used for reasonable<sup>[1]</sup> academic purposes.

## Design
It works by diving the problem space into separate "buckets" characters that each
thread works on in parallel. Each bucket has a base character from which it builds
all possible permutations in the given problems space (character set). The more
threads the program is given, the more it can divide the work<sup>[2]</sup>.

## Building and Running
Building and running this program is fairly easy. You will need the Rust toolchain
installed before continuining. Follow these instructions to build and run it.
```bash
$ git clone https://github.com/MrFlynn/saltine.git
$ cd saltine/
$ cargo build --release
$ ./target/release/saltine --help # Display help menu.
```

## Performance
A randomly generated 6 character (lower case english only) password took 39 minutes 
to crack using 16 threads.
The command used to crack the password is shown below:
```bash
$ ./saltine /path/to/etc_shadow \
    --username=user \
    --threads=16 \
    --alphabet=zyxwvutsrqponmlkjihgfedcba
```

This was run on a virtual machine with an Intel Xeon Platinum 8168 with 16 threads 
and 64GB of RAM allocated<sup>[3]</sup>.

## Notes
**[1]** Please don't copy this program and pass it off as your own work. That is 
considered plagiarism which most universities don't take lightly.
If you are referencing this code for use in any academic work, please cite it.

**[2]** There is a meaningful limitation to the number of threads this program can 
utilize. The number of characters in the problem set defines the upper limit of 
usable threads. For example, for a problem space consisting of all lower-space
english characters

**[3]** This program uses very little RAM. Peak usage was 3.8MB during this benchmark.