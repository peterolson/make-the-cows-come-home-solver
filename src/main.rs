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

    let mut board = Board::from_string("5|3|O__PP____PPH__B");

    // let mut possible_moves = board.get_possible_moves();
    // board = possible_moves[2].0.clone();
    // possible_moves = board.get_possible_moves();
    // board = possible_moves[3].0.clone();
    // possible_moves = board.get_possible_moves();
    // board = possible_moves[0].0.clone();
    // possible_moves = board.get_possible_moves();
    // board = possible_moves[1].0.clone();
    // possible_moves = board.get_possible_moves();
    // board = possible_moves[2].0.clone();
    // possible_moves = board.get_possible_moves();
    // board = possible_moves[1].0.clone();
    // possible_moves = board.get_possible_moves();

    // for move_ in possible_moves {
    //     let (new_board, from, to, puller) = move_;
    //     println!("{} -> {} by {}", from, to, puller);
    //     println!("{}", new_board);
    // }

    let (situation, iterations) = solve(board);

    println!("{}", situation.moves.len());
    //combine_puzzles();
    //generate_files(4, 4);

    print!("Done");
}
