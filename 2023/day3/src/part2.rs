use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calculate(file: &File) -> Option<usize> {
    let file = BufReader::new(file);

    const BUF_LEN: usize = 3;

    #[derive(Debug)]
    enum LineStatus {
        Prev,
        Curr,
        Next
    }

    let mut file_cursor = file.lines();
    let mut lines: LinkedList<String> = LinkedList::new();

    let temp = file_cursor.next().unwrap().unwrap();
    let line_len = temp.len();
    lines.push_back(temp);

    let mut sum: usize = 0;
    let mut curr_line: String;

    'main: loop {
        match file_cursor.next() {
            Some(x) => {
                let x = x.unwrap();
                if lines.len()+1 <= BUF_LEN {
                    curr_line = lines.back().unwrap().to_string();
                    lines.push_back(x);
                } else {
                    lines.pop_front();
                    curr_line = lines.back().unwrap().to_string();
                    lines.push_back(x);
                }
            },
            None => {
                if lines.len() == 2 {
                    break 'main;
                }
                lines.pop_back();
                curr_line = lines.back().unwrap().to_string();
            },
        };


        
        let mut prev_line = String::new();
        let mut next_line = String::new();

        if let Some(l) = lines.front() {
            let l = l.to_string();
            if l != curr_line {
                prev_line = l;
            }
        }

        if let Some(l) = lines.back() {
            let l = l.to_string();
            if l != curr_line {
                next_line = l;
            }
        }

        #[derive(Debug)]
        struct AdjacentItem(LineStatus, usize);
        for (ind, item) in curr_line.chars().enumerate() {

            let mut adjacent_ind: Vec<AdjacentItem> = vec![];
            let mut hor_num = false;

            if !is_symbol(item) {
                continue;
            }

            if let Some(_) = prev_line.chars().nth(0) {
                if ind != 0 {
                    if is_number(prev_line.chars().nth(ind-1).unwrap()) {
                        adjacent_ind.push(AdjacentItem(LineStatus::Prev, ind-1));
                        hor_num = true;
                    }
                }

                if is_number(prev_line.chars().nth(ind).unwrap()) {
                    if !hor_num {
                        adjacent_ind.push(AdjacentItem(LineStatus::Prev, ind));
                        hor_num = true;
                    }
                } else {
                    hor_num = false;
                }

                if ind != line_len-1 {
                    if is_number(prev_line.chars().nth(ind+1).unwrap()) {
                        if !hor_num {
                            adjacent_ind.push(AdjacentItem(LineStatus::Prev, ind+1));
                        }
                    }
                }
            }

            hor_num = false;

            if ind != 0 {
                if is_number(curr_line.chars().nth(ind-1).unwrap()) {
                    adjacent_ind.push(AdjacentItem(LineStatus::Curr, ind-1));
                }
            }

            if ind != line_len-1 {
                if is_number(curr_line.chars().nth(ind+1).unwrap()) {
                    adjacent_ind.push(AdjacentItem(LineStatus::Curr, ind+1));
                }
            }

            if let Some(_) = next_line.chars().nth(0) {
                if ind != 0 {
                    if is_number(next_line.chars().nth(ind-1).unwrap()) {
                        adjacent_ind.push(AdjacentItem(LineStatus::Next, ind-1));
                        hor_num = true;
                    }
                }

                if is_number(next_line.chars().nth(ind).unwrap()) {
                    if !hor_num {
                        adjacent_ind.push(AdjacentItem(LineStatus::Next, ind));
                        hor_num = true;
                    }
                } else {
                    hor_num = false;
                }

                if is_period(item) {
                    hor_num = false;
                }

                if ind != line_len-1 {
                    if is_number(next_line.chars().nth(ind+1).unwrap()) {
                        if !hor_num {
                            adjacent_ind.push(AdjacentItem(LineStatus::Next, ind+1));
                        }
                    }
                }
            }

            if adjacent_ind.len() == 2 {
                let val1 = adjacent_ind.get(0).unwrap();
                let val2 = adjacent_ind.get(1).unwrap();

                let num1 = match val1.0 {
                    LineStatus::Curr =>  isolate_num(&curr_line, val1.1), 
                    LineStatus::Prev =>  isolate_num(&prev_line, val1.1), 
                    LineStatus::Next =>  isolate_num(&next_line, val1.1), 
                };

                let num2 = match val2.0 {
                    LineStatus::Curr =>  isolate_num(&curr_line, val2.1), 
                    LineStatus::Prev =>  isolate_num(&prev_line, val2.1), 
                    LineStatus::Next =>  isolate_num(&next_line, val2.1), 
                };

                sum += num1 * num2;
            }

        }
    }
    return Some(sum);
}


fn is_number(val: char) -> bool {
    match val.to_digit(10) {
        Some(_x) => true,
        None => false,
    }
}

fn is_symbol(val: char) -> bool {
    !is_number(val) && !is_period(val)
}

fn is_period(val: char) -> bool {
    val.eq(&'.')
}


fn isolate_num(line: &String, ind: usize) -> usize {
    let len = line.as_str().len();
    let mut start = ind;
    let mut buf: Vec<char> = vec![];

    for x in (0..ind).rev() {
        if let Some(ch) = line.chars().nth(x) {
            if !is_number(ch) {
                break;
            }
            start = x;
        }
    }

    for x in start..len {
        if let Some(ch) = line.chars().nth(x) {
            if !is_number(ch) {
                break;
            }
            buf.push(ch);
        }
    }

    return parse_num(&buf);
}


fn parse_num(buf: &Vec<char>) -> usize {

    let mut count: usize = 0;
    let base: u32 = 10;
    for (ind, &item) in buf.iter().enumerate() {
        let item = item.to_digit(10).unwrap();
        let counter: u32 = (buf.len()-ind-1).try_into().unwrap();
        count += (base.pow(counter) * item as u32) as usize;
    }
    return count;
}

