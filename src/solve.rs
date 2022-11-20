use std::collections::{HashSet, VecDeque};

use crate::board::{Board, Piece};

#[derive(Clone)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub puller: u8
}


pub struct BoardSituation {
    pub board: Board,
    pub moves: Vec<Move>
}

pub fn solve(board : Board) -> (BoardSituation, isize) {
    let mut encountered_boards : HashSet<Board> = HashSet::new();
    let mut iterations : isize = 0;

    let mut search_queue : VecDeque<BoardSituation> = VecDeque::new();
    search_queue.push_back(BoardSituation {
        board: board.clone(),
        moves: Vec::new()
    });

    while !search_queue.is_empty() {
        let situation = search_queue.pop_front().unwrap();
        let board = situation.board.clone();
        iterations += 1;
        if iterations % 100000 == 0 {
            println!("Iteration {}, {} moves, {} in queue", iterations, situation.moves.len(), search_queue.len());
        }
        if board.is_solved() {
            return (situation, iterations);
        }
        if encountered_boards.contains(&board) {
            continue;
        }
        let possible_boards = board.get_possible_moves();
        for (possible_board, from, to, puller) in possible_boards {
            let mut new_moves = situation.moves.clone();
            new_moves.push(Move {
                from: from,
                to: to,
                puller: puller
            });
            search_queue.push_back(BoardSituation { board: possible_board, moves: new_moves });
        }
        encountered_boards.insert(board);
    }

    (BoardSituation {
        board: board.clone(),
        moves: Vec::new()
    }, -1)
}

impl BoardSituation {
    pub fn uses_all_pieces(&self) -> bool {
        let mut encountered_indices : HashSet<u8> = HashSet::new();
        for m in &self.moves {
            encountered_indices.insert(m.from);
            encountered_indices.insert(m.to);
            encountered_indices.insert(m.puller);
        }
        for i in 0..self.board.pieces.len() {
            let piece = self.board.pieces[i];
            if piece == Piece::Empty || piece == Piece::Blank {
                continue;
            }
            if !encountered_indices.contains(&(i as u8)) {
                return false;
            }
        }
        return true;
    }

    pub fn uses_all_rows_columns(&self) -> bool {
        let mut encountered_rows : HashSet<u8> = HashSet::new();
        let mut encountered_columns : HashSet<u8> = HashSet::new();
        for m in &self.moves {
            encountered_rows.insert(m.from / self.board.width);
            encountered_rows.insert(m.to / self.board.width);
            encountered_rows.insert(m.puller / self.board.width);
            encountered_columns.insert(m.from % self.board.width);
            encountered_columns.insert(m.to % self.board.width);
            encountered_columns.insert(m.puller % self.board.width);
        }
        for i in 0..self.board.height {
            if !encountered_rows.contains(&i) {
                return false;
            }
        }
        for i in 0..self.board.width {
            if !encountered_columns.contains(&i) {
                return false;
            }
        }
        return true;
    }

    pub fn is_elegant(&self) -> bool {
        self.uses_all_pieces() && self.uses_all_rows_columns()
    }
}