use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn calculate(file: &File) -> Option<usize> {

    let file = BufReader::new(file); 

    // red, green, blue
    struct Digits(usize, usize, usize);

    const DIGITS_MAX: Digits = Digits(12, 13, 14);

    let mut game_num: usize = 1;
    let mut result: usize = 0;
    'line: for line in file.lines() {
        let line = match line {
            Ok(x) => x,
            Err(_) => {
                return None;
            }
        };
        let sets: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = sets[1].split(";").collect();

        for (_, &set) in sets.iter().enumerate() {

            for choice in set.trim().split(",").collect::<Vec<&str>>()  {
                let choice: Vec<&str> = choice.trim().split(" ").collect();
                let temp = choice[0].parse::<usize>().expect("invalid number");
                match choice[1] {
                    "red" => {
                        if temp > DIGITS_MAX.0 {
                            game_num += 1;
                            continue 'line;
                        }
                    },
                    "green" => {
                        if temp > DIGITS_MAX.1 {
                            game_num += 1;
                            continue 'line;
                        }
                    },
                    "blue" => {
                        if temp > DIGITS_MAX.2 {
                            game_num += 1;
                            continue 'line;
                        }
                    },
                    _ => {
                        println!("nothing matched");
                    }
                }
            }
        }

        result += game_num;
        game_num += 1;
    }

   return Some(result);
    
}

