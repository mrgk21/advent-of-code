use std::{fs::File, io::Seek};

pub mod part1;
pub mod part2;

fn main() {

    let mut file = File::open("./src/data.txt").expect("unable to open the file");

    let result1 = part1::calculate(&file);
    let result1 = match result1 {
        Some(x) => x,
        None => 0
    };

    file.rewind().unwrap();

    let result2 = part2::calculate(&file);
    let result2 = match result2 {
        Some(x) => x,
        None => 0
    };

    println!("{result1} {result2}");
}
