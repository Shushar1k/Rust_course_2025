#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file_1 = File::open(&args[1]).unwrap();
    let reader_1 = BufReader::new(file_1);
    let data_1: HashSet<String> = reader_1.lines().collect();
    
    let file_2 = File::open(&args[2]).unwrap();
    let reader_2 = BufReader::new(file_2);
    let data_2: HashSet<String> = reader_2.lines().collect();
    let mut result: HashSet<String> = HashSet::new();

    for i in data_1 {
        for j in data_2 {
            if i == j {
                result.insert(i);
            }
        }
    }

    for i in result {
        println!("{}", i);
    }
}
