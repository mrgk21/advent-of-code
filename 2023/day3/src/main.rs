use std::{fs::File, io::Seek};

pub mod part1;
pub mod part2;

fn main() {
    let mut file = File::open("src/data.txt").expect("cannot open file");

    let sum = part1::calculate(&file);
    let sum = match sum {
        Some(x) => x,
        None => 0
    };

    file.rewind().unwrap();

    let sum2 = part2::calculate(&file);
    let sum2 = match sum2 {
        Some(x) => x,
        None => 0
    };

    println!("{sum} {sum2}");
}
