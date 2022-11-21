use crate::{
    generate::{generate_boards, generate_file, generate_files},
    solve::solve,
};

mod board;
mod generate;
mod settings;
mod solve;

fn main() {
    println!("Hello, world!");

    generate_files(3, 3);

    print!("Done");
}
