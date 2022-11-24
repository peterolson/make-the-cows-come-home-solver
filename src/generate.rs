use std::collections::{HashSet,};
use std::fs::{ File};
use std::io::Write;


use crate::board::{Board, Piece};
use crate::reverse_solver::reverse_solve;
use crate::settings::HEXAGONAL_MODE;
use crate::solve::{Solution};

const DIRECTORY: &str = if HEXAGONAL_MODE { "hex" } else { "rect" };


pub fn generate_puzzles(width: u8, height: u8) {
    let mut initial_boards : Vec<Board> = Vec::new();
    for barn_count in 1..3 {
        for house_count in 0..2 {
            let mut max_empty = 3;
            if barn_count == 2 {
                max_empty = 2;
            }
            for empty_count in 0..max_empty {
                let mut boards = get_initial_boards(width, height, barn_count, house_count, empty_count);
                println!("{} boards with {} barns, {} houses, {} empty", boards.len(), barn_count, house_count, empty_count);
                initial_boards.append(&mut boards);
            }
        }
    }
    println!("Initial boards: {}", initial_boards.len());

    let length = width * height;

    let mut piece_combinations : Vec<(Board, u8, u8)> = Vec::new();
    for board in initial_boards {
        let max_cow_count : u8 = 5;
        let mut max_person_count : u8 = 5;
        let houses = board.count_piece(Piece::House);
        let barns = board.count_piece(Piece::Barn);
        let empty = board.count_piece(Piece::Empty);
        if houses == 0 {
            max_person_count = 3;
        }
        
        for cows in 1..(max_cow_count + 1) {
            for people in 1..(max_person_count + 1) {
                if(cows + people + barns + houses + empty) > length - 2 
                {
                    // must have at least 2 empty spaces
                    continue;
                }
                if cows + people > length / 2 {
                    // moving pieces can't fill more than half the board
                    continue;
                }
                piece_combinations.push((board.clone(), cows, people));
            }
        }
    }

    let mut output_rows : Vec<(Board, String, u8, usize)> = Vec::new();

    let mut completed = 0;
    let total = piece_combinations.len();
    for (board, cows, people) in piece_combinations {
        let percent = (completed as f32 / total as f32) * 100.0;
        let houses = board.count_piece(Piece::House);
        let barns = board.count_piece(Piece::Barn);
        let empty = board.count_piece(Piece::Empty);
        let description_string = format!("{}_{} {}_{}_{}_{}_{}", width, height, cows, people, houses, barns, empty);
        println!("{}\t{} / {} = {}%", description_string, completed, total, percent);
        let (puzzle_board, reverse_solution, iterations) = reverse_solve(board.clone(), cows, people);
        let solution = Solution {
            moves: reverse_solution.moves.into_iter().rev().collect(),
            move_count: reverse_solution.move_count,
            tree_size: iterations,
            can_be_solved: true,
        };
        let is_elegant = solution.is_elegant(&puzzle_board);
        //println!("Board: {} Elegant? {}  {} moves, {} iterations", puzzle_board.to_string(), is_elegant, reverse_solution.move_count, iterations);
        if is_elegant {
            output_rows.push((puzzle_board, description_string, reverse_solution.move_count, iterations));
        }
        completed += 1;
    }

    // write to file
    let lines = output_rows
        .iter()
        .map(|row| format!("{}\t{}\t{}\t{}", row.0.to_string(), row.1, row.2, row.3))
        .collect::<Vec<String>>();

    let file_name = format!("{}_{}.txt", width, height);
    let data = lines.join("\n");
    let mut f = File::create(format!("{}/{}", DIRECTORY, file_name))
        .expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");

}

fn get_initial_boards(width: u8, height: u8, barn_count : u8, house_count: u8, empty_count: u8) -> Vec<Board> {
    return  generate_boards(width, height, 0, barn_count, 0, house_count, empty_count);
}

pub fn generate_boards(
    width: u8,
    height: u8,
    cow_count: u8,
    barn_count: u8,
    person_count: u8,
    house_count: u8,
    empty_count: u8,
) -> Vec<Board> {
    let length = width * height;
    let strings = generate_with_prefix(
        length,
        "",
        cow_count,
        barn_count,
        person_count,
        house_count,
        empty_count,
    );
    let mut boards: Vec<Board> = Vec::new();
    let mut encountered_variants: HashSet<Board> = HashSet::new();
    for string in strings {
        let full_string = format!("{}|{}|{}", width, height, string);
        let board = Board::from_string(&full_string);
        if encountered_variants.contains(&board) {
            continue;
        }
        let variants = board.get_symmetric_variants();
        for variant in variants {
            encountered_variants.insert(variant);
        }
        boards.push(board);
    }
    boards
}

fn generate_with_prefix(
    length: u8,
    prefix: &str,
    cow_count: u8,
    barn_count: u8,
    person_count: u8,
    house_count: u8,
    empty_count: u8,
) -> Vec<String> {
    if cow_count + barn_count + person_count + house_count + empty_count
        > (length - prefix.len() as u8)
    {
        return Vec::new();
    }
    if prefix.len() == length as usize {
        return vec![prefix.to_string()];
    }
    let mut boards: Vec<String> = Vec::new();
    if cow_count > 0 {
        let mut new_prefix = prefix.to_string();
        new_prefix.push('O');
        boards.append(&mut generate_with_prefix(
            length,
            &new_prefix,
            cow_count - 1,
            barn_count,
            person_count,
            house_count,
            empty_count,
        ));
    }
    if barn_count > 0 {
        let mut new_prefix = prefix.to_string();
        new_prefix.push('B');
        boards.append(&mut generate_with_prefix(
            length,
            &new_prefix,
            cow_count,
            barn_count - 1,
            person_count,
            house_count,
            empty_count,
        ));
    }
    if person_count > 0 {
        let mut new_prefix = prefix.to_string();
        new_prefix.push('P');
        boards.append(&mut generate_with_prefix(
            length,
            &new_prefix,
            cow_count,
            barn_count,
            person_count - 1,
            house_count,
            empty_count,
        ));
    }
    if house_count > 0 {
        let mut new_prefix = prefix.to_string();
        new_prefix.push('H');
        boards.append(&mut generate_with_prefix(
            length,
            &new_prefix,
            cow_count,
            barn_count,
            person_count,
            house_count - 1,
            empty_count,
        ));
    }
    if empty_count > 0 {
        let mut new_prefix = prefix.to_string();
        new_prefix.push('E');
        boards.append(&mut generate_with_prefix(
            length,
            &new_prefix,
            cow_count,
            barn_count,
            person_count,
            house_count,
            empty_count - 1,
        ));
    }
    let mut new_prefix = prefix.to_string();
    new_prefix.push('_');
    boards.append(&mut generate_with_prefix(
        length,
        &new_prefix,
        cow_count,
        barn_count,
        person_count,
        house_count,
        empty_count,
    ));
    boards
}
