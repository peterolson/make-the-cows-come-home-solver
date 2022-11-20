use std::collections::{HashSet, VecDeque};

use crate::board::Board;

pub fn solve(board : Board) -> (i8, usize) {
    let mut encountered_boards : HashSet<Board> = HashSet::new();
    let mut iterations : usize = 0;

    let mut search_queue : VecDeque<(Board, i8)> = VecDeque::new();
    search_queue.push_back((board, 0));

    while !search_queue.is_empty() {
        let (board, moves) = search_queue.pop_front().unwrap();
        iterations += 1;
        if iterations % 100000 == 0 {
            println!("Iteration {}, {} moves, {} in queue", iterations, moves, search_queue.len());
        }
        if board.is_solved() {
            return (moves, iterations);
        }
        if encountered_boards.contains(&board) {
            continue;
        }
        let possible_boards = board.get_possible_moves();
        for possible_board in possible_boards {
            search_queue.push_back((possible_board, moves + 1));
        }
        encountered_boards.insert(board);
    }

    (-1, iterations)
}