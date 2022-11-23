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

    //let mut solution_map = HashMap::new();

    // //combine_puzzles();
    // generate_files(3, 2);
    // generate_files(4, 2);
    // generate_files(3,3);
    // generate_files(4, 3);
    // generate_files(5, 3, &mut solution_map);

    // let board = Board::from_string("5|3|PO__P____HP_OBP");

    // let solution = solve(board, &mut solution_map);

    // println!("Solution: {:?} {}", solution.moves, solution.moves.len());

    // let board = Board::from_string("4|3|O_PBB__HPO_P");
    // let solution = solve(board, &mut solution_map);

    // let board = Board::from_string("5|3|PO_OPHP____B_P_");
    // let solution = solve(board, &mut solution_map);

    let board = Board::from_string("5|3|OO_O_O______O__");

    let reverse_moves = board.get_reverse_moves(0);
    println!("Reverse moves: {:?}", reverse_moves);

    let possible_previous_boards = board.get_possible_previous_boards();
    for (board, from, to, puller) in possible_previous_boards {
        println!("{} {} {}", from, to, puller);
        println!("{}", board);
    }

    print!("Done");
}
