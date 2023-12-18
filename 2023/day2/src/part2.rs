use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn calculate(file: &File) -> Option<usize> {

    let file = BufReader::new(file); 

    // red, green, blue
    #[derive(Default, Debug)]
    struct Digits(usize, usize, usize);

    let mut result: usize = 0;
    for line in file.lines() {
        let line = match line {
            Ok(x) => x,
            Err(_) => {
                return None;
            }
        };
        let sets: Vec<&str> = line.split(":").collect();
        let sets: Vec<&str> = sets[1].split(";").collect();

        let mut min_digits: Digits = Digits::default();
        for (_, &set) in sets.iter().enumerate() {


            for choice in set.trim().split(",").collect::<Vec<&str>>()  {
                let choice: Vec<&str> = choice.trim().split(" ").collect();
                let temp = choice[0].parse::<usize>().expect("invalid number");
                match choice[1] {
                    "red" => {
                        if temp > min_digits.0 {
                            min_digits.0 = temp;
                        }
                    },
                    "green" => {
                        if temp > min_digits.1 {
                            min_digits.1 = temp;
                        }
                    },
                    "blue" => {
                        if temp > min_digits.2 {
                            min_digits.2 = temp;
                        }
                    },
                    _ => {
                        println!("nothing matched {}", choice[1]);
                    }
                }
            }
        }

        result += min_digits.0 * min_digits.1 * min_digits.2;
    }

   return Some(result);
    
}

