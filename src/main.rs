use std::collections::HashMap;

use crate::combine::combine_puzzles;

mod board;
mod combine;
mod generate;
mod reverse_solver;
mod settings;
mod solve;

fn main() {
    println!("Hello, world!");

    combine_puzzles();

    // generate::generate_puzzles(3,2);
    // generate::generate_puzzles(4,2);
    // generate::generate_puzzles(3,3);
    // generate::generate_puzzles(4,3);
    // generate::generate_puzzles(5,3);


    // generate::generate_puzzles(6,3);
    // generate::generate_puzzles(4,4);
    // generate::generate_puzzles(5,4);
    // generate::generate_puzzles(6,4);
    // generate::generate_puzzles(5,5);
    // generate::generate_puzzles(6,5);
    // generate::generate_puzzles(6,6);

    // let board = board::Board::from_string("4|4|BH_O_POOEPOOP___");
    // let mut hashmap = HashMap::new();

    // let solution = solve::solve(board, &mut hashmap);

    // println!("Solution: {:?} {}", solution.moves, solution.move_count);

    // let board = board::Board::from_string("6|4|_BE______E__H___");

    // let reverse_solution = reverse_solver::reverse_solve(board, 5, 3);

    // println!("{}", reverse_solution.0);
    // print!("{}", reverse_solution.0.to_string());
    // println!(
    //     "Solution: {:?} {}",
    //     reverse_solution.1.moves,
    //     reverse_solution.1.moves.len()
    // );

    print!("Done");
}
