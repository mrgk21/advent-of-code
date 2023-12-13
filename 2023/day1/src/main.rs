use std::fs;

pub mod part1;

fn main() {
    let file = fs::File::open("./src/data.txt").expect("unable to open the file");

    let count1 = part1::calculate(&file);
    let count1 = match count1 {
        Option::None => {
            println!("Bad count");
            0
        },
        Option::Some(x) => x
    };

    println!("count1: {}", count1);    
}



