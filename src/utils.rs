extern crate rand;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use self::rand::{thread_rng, Rng};

pub fn read_file(path: &str) -> Vec<String> {
    let file = match File::open(path) { Ok(f) => f, Err(..) => panic!("could not open file"), };
    let reader = BufReader::new(&file);
    reader.lines().filter_map(|result| result.ok()).collect::<Vec<String>>()
}

pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut x = vec![0u8; n];
    rng.fill_bytes(&mut x);
    x
}

pub fn print_bytes(x: &[u8]) {
    for i in 0..x.len() {
        print!("{:02X} ", x[i]);
    }
    println!("");
}
