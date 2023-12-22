use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calculate(file: &File) -> Option<usize> {

    let file = BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    for res in file.lines() {
        match res {
            Ok(x) => lines.push(x),
            Err(_) => return None,
        };
    }
    let mut sum = 0;

    'lines: for (line_ind, line) in lines.iter().enumerate() {
        
        let line = line.trim().as_bytes().to_vec();
        let mut buf: Vec<u8> = vec![];
        let mut num_found = false;

        for ind in 0..line.len() {
            let item = line[ind];

            if !is_number(item) {
                if num_found && buf.len() > 0 {
                    let num = parse_num(&buf);
                    sum += num;
                    num_found = false;
                }
                buf.clear();   
                continue;
            }

            if ind != 0 {
                if is_symbol(line[ind-1]) {
                    num_found = true;
                }

                if line_ind != 0 {
                    if is_symbol(lines[line_ind-1].as_bytes()[ind-1]) {
                        num_found = true;
                    }
                }

                if line_ind != lines.len()-1 {
                    if is_symbol(lines[line_ind+1].as_bytes()[ind-1]) {
                        num_found = true;
                    }
                }
            }
        
            if ind != line.len()-1 {
                if is_symbol(line[ind+1]) {
                    num_found = true;
                }

                if line_ind != 0 {
                    if is_symbol(lines[line_ind-1].as_bytes()[ind+1]) {
                        num_found = true;
                    }
                }

                if line_ind != lines.len()-1 {
                    if is_symbol(lines[line_ind+1].as_bytes()[ind+1]) {
                        num_found = true;
                    }
                }
            }

            if line_ind != 0 {
                if is_symbol(lines[line_ind-1].as_bytes()[ind]) {
                    num_found = true;
                }
            }

            if line_ind != lines.len()-1 {
                if is_symbol(lines[line_ind+1].as_bytes()[ind]) {
                    num_found = true;
                }
            }

            buf.push(item);
        }
        if num_found && buf.len() > 0 {
            let num = parse_num(&buf);
            sum += num;
            num_found = false;
        }
        buf.clear();
    }

    return Some(sum);
}

fn parse_num(buf: &Vec<u8>) -> usize {

    let mut count: usize = 0;
    let base: u32 = 10;
    for (ind, &item) in buf.iter().enumerate() {
        let item = item - 48;
        let counter: u32 = (buf.len()-ind-1).try_into().unwrap();
        count += (base.pow(counter) * item as u32) as usize;
    }
    return count;
}

pub fn is_number(val: u8) -> bool {
    val >= b'0' && val <= b'9'
}

pub fn is_symbol(val: u8) -> bool {
    !is_number(val) && !is_period(val)
}

pub fn is_period(val: u8) -> bool {
    val == b'.'
}