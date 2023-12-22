use std::fs::File;

pub mod part1;

fn main() {
    let file = File::open("src/data.txt").expect("cannot open file");

    let sum = part1::calculate(&file);
    let sum = match sum {
        Some(x) => x,
        None => 0
    };

    println!("{sum}");
}
