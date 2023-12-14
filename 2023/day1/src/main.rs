use std::fs;
use std::io::{Seek};
pub mod part1;
pub mod part2;

fn main() {
    let mut file = fs::File::open("./src/data.txt").expect("unable to open the file");

    let count1 = part1::calculate(&file);
    let count1 = match count1 {
        Option::None => {
            println!("Bad count");
            0
        },
        Option::Some(x) => x
    };

    if let Result::Ok(_) = file.rewind() {
        println!("File rewinded to original value");
    }

    let count2 = part2::calculate(&file);
    let count2 = match count2 {
        Option::None => {
            println!("Bad count");
            0
        },
        Option::Some(x) => x
    };

    println!("count2: {count2}, count1 {count1}");    
}



