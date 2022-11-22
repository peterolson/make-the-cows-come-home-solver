use crate::{
    combine::combine_puzzles,
    generate::{generate_boards, generate_file, generate_files},
    solve::solve,
};

mod board;
mod combine;
mod generate;
mod settings;
mod solve;

fn main() {
    println!("Hello, world!");

    combine_puzzles();
    //generate_files(4, 4);

    print!("Done");
}
