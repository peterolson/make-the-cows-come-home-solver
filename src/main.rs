use crate::{generate::{generate_boards, generate_file, generate_files}, solve::solve};

mod board;
mod solve;
mod generate;

fn main() {
    println!("Hello, world!");

    generate_files(5, 3);

    print!("Done");

}

