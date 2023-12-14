use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::{from_utf8};

pub fn calculate(file: &File) -> Option<usize> {
    
    let file = BufReader::new(file); 
    
    #[derive(Debug)] 
    struct Digits(u8, u8);

    let mut count: usize = 0;

    for line in file.lines() {
        let bts = match line {
            Result::Err(_x) => {
                return None;
            },
            Result::Ok(val) => val 
        };

        let bts = bts.as_bytes().to_vec();
        let mut bts_rev = bts.clone(); 
        rev_vect(&mut bts_rev);


        let mut digits = Digits(0,0);

        let trailing_ind = 0;
        'outer1: for (ind, &item) in bts.iter().enumerate() {
            let mut increment = trailing_ind;
            if ind > 0 {
            loop {
                if increment == ind {
                    break;
                }
                let part = from_utf8(&bts[increment..ind]).unwrap();
                let part2 = match part {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => 0
                };
                if part2 > 0 {
                    digits.0 = part2;
                    break 'outer1;
                }
                increment += 1;
            }

            }
            if item >= b'0' && item <= b'9' {
                digits.0 = item - 48;
                break 'outer1;
            }
        } 

        'outer2: for (ind, &item) in bts_rev.iter().enumerate() {
            let mut increment = trailing_ind;
            if ind > 0 {
            loop {
                let part = from_utf8(&bts_rev[increment..ind]).unwrap();
                let part2 = match part { 
                    "eno" => 1,
                    "owt" => 2,
                    "eerht" => 3,
                    "ruof" => 4,
                    "evif" => 5,
                    "xis" => 6,
                    "neves" => 7,
                    "thgie" => 8,
                    "enin" => 9,
                    _ => 0
                };
                if part2 > 0 {
                    digits.1 = part2;
                    break 'outer2;
                }
                increment += 1;
                if increment == ind {
                    break;
                }
            }

            }
            if item >= b'0' && item <= b'9' {
                digits.1 = item - 48;
                break 'outer2;
            }
        } 
        count += (10 * digits.0 as usize) + digits.1 as usize; 
    }
    return Some(count);
}

fn rev_vect(s: &mut Vec<u8>) -> &Vec<u8> {
    let len = s.len();

    for count in 0..len/2 {
        let temp = s[count];
        s[count] = s[len-count-1];
        s[len-count-1] = temp;

        if count == len-1 {
            break;
        }
    }
    s
}

