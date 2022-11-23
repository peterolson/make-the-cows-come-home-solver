use std::collections::{HashMap, HashSet};

use crate::{
    board::Board,
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

    // //combine_puzzles();
    generate_files(3, 2);
    generate_files(4, 2);
    generate_files(3,3);
    generate_files(4, 3);
    // generate_files(4, 3);



    print!("Done");
}
