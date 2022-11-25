use std::collections::{HashSet, HashMap};

use crate::board::{Board, Piece};

#[derive(Clone, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub puller: u8
}

#[derive(Clone)]
pub struct Solution {
    pub moves: Vec<Move>,
    pub move_count: u8,
    pub tree_size: usize,
    pub can_be_solved: bool
}

pub fn solve(board : Board, solution_map : &mut HashMap<Board, Solution>) -> Solution {
    let mut encountered_boards = HashSet::new();
    solve_internal(board, solution_map, &mut encountered_boards)
}

fn solve_internal(board : Board, solution_map : &mut HashMap<Board, Solution>, encountered_boards : &mut HashSet<Board>) -> Solution {
    
    if board.is_solved() {
        let solution = Solution {
            moves: Vec::new(),
            move_count: 0,
            tree_size: 1,
            can_be_solved: true
        };
        solution_map.insert(board, solution.clone());
        return solution;
    }

    if solution_map.contains_key(&board) {
        return solution_map.get(&board).unwrap().clone();
    }
   

    encountered_boards.insert(board.clone());

    let mut tree_size = 1;
    let mut can_be_solved = false;
    let mut best_moves : Vec<Move> = Vec::new();
    let mut best_moves_length : u8 = 120;

    let possible_board_moves = board.get_possible_moves();
    for (possible_board, from, to, puller) in possible_board_moves {
        if encountered_boards.contains(&possible_board) {
            continue;
        }
        let solution = solve_internal(possible_board.clone(), solution_map, encountered_boards);

        if !solution.can_be_solved {
            continue;
        }      

        tree_size += solution.tree_size;
        
        let solution_len = solution.moves.len() as u8;

        can_be_solved = true;
        if solution_len < best_moves_length {
            best_moves_length = solution_len + 1;
            best_moves = solution.moves.clone();
            best_moves.insert(0, Move {
                from: from,
                to: to,
                puller: puller
            });
        }
    }

    encountered_boards.remove(&board);

    let best_solution = Solution { moves: best_moves, move_count: best_moves_length, tree_size: tree_size, can_be_solved: can_be_solved };
    solution_map.insert(board, best_solution.clone());
    return best_solution;
}

impl Solution {
    pub fn uses_all_pieces(&self, board: &Board) -> bool {
        let mut encountered_indices : HashSet<u8> = HashSet::new();
        for m in &self.moves {
            encountered_indices.insert(m.from);
            encountered_indices.insert(m.to);
            encountered_indices.insert(m.puller);
        }
        for i in 0..board.pieces.len() {
            let piece = board.pieces[i];
            if piece == Piece::Empty || piece == Piece::Blank {
                continue;
            }
            if !encountered_indices.contains(&(i as u8)) {
                return false;
            }
        }
        return true;
    }

    pub fn uses_all_rows_columns(&self, board: &Board) -> bool {
        let mut encountered_rows : HashSet<u8> = HashSet::new();
        let mut encountered_columns : HashSet<u8> = HashSet::new();
        for m in &self.moves {
            encountered_rows.insert(m.from / board.width);
            encountered_rows.insert(m.to / board.width);
            encountered_rows.insert(m.puller / board.width);
            encountered_columns.insert(m.from % board.width);
            encountered_columns.insert(m.to % board.width);
            encountered_columns.insert(m.puller % board.width);
        }
        for i in 0..board.height {
            if !encountered_rows.contains(&i) {
                return false;
            }
        }
        for i in 0..board.width {
            if !encountered_columns.contains(&i) {
                return false;
            }
        }
        return true;
    }

    pub fn is_elegant(&self, board: &Board) -> bool {
        self.uses_all_pieces(&board) && self.uses_all_rows_columns(&board)
    }
}