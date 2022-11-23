use std::collections::{HashSet, VecDeque};

use crate::{
    board::{Board, Piece},
    solve::Move,
};

#[derive(Clone)]
pub struct ReverseSolution {
    pub moves: Vec<Move>,
    pub move_count: u8,
}

pub fn reverse_solve(
    board: Board,
    max_cow_count: u8,
    max_person_count: u8,
) -> (Board, ReverseSolution) {
    let mut encountered_boards: HashSet<Board> = HashSet::new();

    let mut board_queue: VecDeque<(Board, ReverseSolution)> = VecDeque::new();

    let mut most_moves = 0;
    let mut board_with_most_moves = board.clone();
    let mut best_reverse_solution = ReverseSolution {
        moves: Vec::new(),
        move_count: 0,
    };

    board_queue.push_back((board_with_most_moves.clone(), best_reverse_solution.clone()));
    encountered_boards.insert(board.clone());

    while board_queue.len() > 0 {
        let next_item = board_queue.pop_front().unwrap();
        let board = next_item.0;
        let reverse_solution = next_item.1;
        let previous_boards = board.get_possible_previous_boards();

        for (board, from, to, puller) in previous_boards {
            if encountered_boards.contains(&board) {
                continue;
            }
            if board.count_piece(Piece::Cow) > max_cow_count
                || board.count_piece(Piece::Person) > max_person_count
            {
                continue;
            }
            let mut new_moves = reverse_solution.moves.clone();
            new_moves.push(Move { from, to, puller });
            let new_solution = ReverseSolution {
                moves: new_moves,
                move_count: reverse_solution.move_count + 1,
            };
            if new_solution.move_count > most_moves {
                most_moves = new_solution.move_count;
                board_with_most_moves = board.clone();
                best_reverse_solution = new_solution.clone();
            }
            encountered_boards.insert(board.clone());
            board_queue.push_back((board, new_solution));
        }
    }

    println!("Boards traversed {}", encountered_boards.len());
    return (board_with_most_moves, best_reverse_solution);
}
