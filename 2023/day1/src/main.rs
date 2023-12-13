use std::fs;
use std::io::{BufReader, BufRead};

fn main() {
    #[derive(Debug)] 
    struct Digit(u8, u8);

    let mut count: usize = 0;

    let file = fs::File::open("./src/data.txt").expect("unable to open the file");
    let file = BufReader::new(file); 

    for line in file.lines() {
        let bts = match line {
            Result::Err(x) => {
                println!("invalid data {}", x);
                return;
            },
            Result::Ok(val) => val 
        };

        let bts = bts.as_bytes();

        let mut digit = Digit(0,0);

        for (_i, &item) in bts.iter().enumerate() {
            if item >= b'0' && item <= b'9' {
                digit.0 = item - 48;
                break;
            }
        }

        for (_i, &item) in bts.iter().rev().enumerate() {
            if item >= b'0' && item <= b'9' {
                digit.1 = item - 48;
                break;
            }
        }
        count += (10 * digit.0 as usize) + digit.1 as usize;
    }
    println!("{}", count);
}

