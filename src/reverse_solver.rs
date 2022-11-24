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
) -> (Board, ReverseSolution, usize) {
    let mut encountered_boards: HashSet<Board> = HashSet::new();

    let mut board_queue: VecDeque<(Board, ReverseSolution)> = VecDeque::new();

    if max_person_count > 0 && board.count_piece(Piece::House) == 0 {
        choose_people_locations(&board, max_person_count, &mut board_queue);
    }

    let mut most_moves = 0;
    let mut board_with_most_moves = board.clone();
    let mut best_reverse_solution = ReverseSolution {
        moves: Vec::new(),
        move_count: 0,
    };

    board_queue.push_back((board_with_most_moves.clone(), best_reverse_solution.clone()));
    encountered_boards.insert(board.clone());

    let mut iterations = 0;

    while board_queue.len() > 0 {
        iterations += 1;
        if iterations % 100000 == 0 {
            println!("Iterations: {}. In board queue: {}", iterations, board_queue.len());
        }
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

    return (board_with_most_moves, best_reverse_solution, iterations);
}

fn choose_people_locations(board: &Board, max_person_count: u8, board_queue: &mut VecDeque<(Board, ReverseSolution)>) {
    let mut blank_indices : Vec<u8> = Vec::new();
    for i in 0..(board.width * board.height) {
        if board.pieces[i as usize] == Piece::Blank {
            blank_indices.push(i);
        }
    }

    // get all unique combinations of blank indices with length max_person_count
    let mut combinations : Vec<Vec<u8>> = Vec::new();
    get_combinations(&mut combinations, &blank_indices, max_person_count as usize, 0, Vec::new());

    for combination in combinations {
        let mut new_board = board.clone();
        for index in combination {
            new_board.pieces[index as usize] = Piece::Person;
        }
        board_queue.push_back((new_board, ReverseSolution { moves: Vec::new(), move_count: 0 }));
    }
}

fn get_combinations(combinations: &mut Vec<Vec<u8>>, blank_indices: &Vec<u8>, max_count: usize, index: usize, current_combination: Vec<u8>) {
    if current_combination.len() == max_count {
        combinations.push(current_combination);
        return;
    }
    if index == blank_indices.len() {
        return;
    }
    let mut new_combination = current_combination.clone();
    new_combination.push(blank_indices[index]);
    get_combinations(combinations, blank_indices, max_count, index + 1, new_combination);
    get_combinations(combinations, blank_indices, max_count, index + 1, current_combination);
}