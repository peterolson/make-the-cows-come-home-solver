extern crate rand;
use std::{fs, collections::HashMap};

use crate::board::Board;

pub fn combine_puzzles() {
    let directories = vec!["rect", "hex"];

    let mut rows: Vec<(String, u8, u32, f32)> = Vec::new();

    // loop through the folders in each directory
    for directory in directories {
        // loop through the files in each folder
        let files = fs::read_dir(directory).unwrap();
        for file in files {
            let file = file.unwrap().path();
            let file = file.to_str().unwrap();

            // read the file
            let contents = fs::read_to_string(file).unwrap();
            // split the file into lines
            let lines: Vec<&str> = contents.lines().collect();
            
            let mut line_number = 0;
            // loop through the lines
            for line in lines {
                line_number += 1;
                if line_number % 10000 == 0 {
                    println!("{} {}", file, line_number);
                }
                let columns: Vec<&str> = line.split('\t').collect();
                let board = columns[0].to_string();
                let moves = columns[2].parse::<u8>().unwrap();
                let iterations = columns[3].parse::<u32>().unwrap();
                let person_count = board.matches('P').count();
                let cow_count = board.matches('O').count();
                let mover_count = cow_count + person_count;
                let difficulty =
                    (moves as f32) * f32::log2(iterations as f32) / (mover_count as f32);
                let h_r = if directory == "rect" { "R" } else { "H" };
                let board_string = format!("{}~{}{}", moves, h_r, board);
                if (moves as f32) / (mover_count as f32) < 2.0 {
                    // it's not interesting when the pieces can move directly into the goal
                    continue;
                }
                if difficulty < (30 as f32) && board.contains("E") {
                    // it's not interesting when the empty space is not necessary
                    // too expensive to check if it's necessary, so just ignoring for low difficulties
                    continue;
                }
                rows.push((board_string, moves, iterations, difficulty));
            }
        }
    }

    // sort the rows by difficulty
    rows.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    rows.dedup_by(|a, b|a.0 == b.0);
    rows.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    rows.dedup_by(|a, b|(a.3 - b.3).abs() < 0.001);

    let min_difficulty = rows[0].3;
    let max_difficulty = rows[rows.len() - 1].3;
    println!("Max difficulty: {}", max_difficulty);
    println!("Min difficulty: {}", min_difficulty);
    let difficulty_span = max_difficulty - min_difficulty;

    let easy_bucket : Vec<(String, u8, u32, f32)> = rows.iter().filter(|x| x.3 - min_difficulty < difficulty_span *0.15).map(|x| x.clone()).collect();
    let moderate_bucket : Vec<(String, u8, u32, f32)> = rows.iter().filter(|x| x.3 - min_difficulty >= difficulty_span * 0.20 && x.3 - min_difficulty  < difficulty_span * 0.35).map(|x| x.clone()).collect();
    let hard_bucket : Vec<(String, u8, u32, f32)> = rows.iter().filter(|x| x.3 - min_difficulty >= difficulty_span * 0.40 && x.3 - min_difficulty  < difficulty_span * 0.6).map(|x| x.clone()).collect();
    let insane_bucket : Vec<(String, u8, u32, f32)> = rows.iter().filter(|x| x.3 - min_difficulty >= difficulty_span * 0.65).map(|x| x.clone()).collect();


    println!("Easy: {}", easy_bucket.len());
    println!("Moderate: {}", moderate_bucket.len());
    println!("Hard: {}", hard_bucket.len());
    println!("Insane: {}", insane_bucket.len());

    let puzzles_per_bucket = 30;

    let easy_reduced = sample_balance_board_type(easy_bucket, puzzles_per_bucket);
    let moderate_reduced = sample_balance_board_type(moderate_bucket, puzzles_per_bucket);
    let hard_reduced = sample_balance_board_type(hard_bucket, puzzles_per_bucket);
    let insane_reduced = sample_balance_board_type(insane_bucket, puzzles_per_bucket);

    print_puzzles(easy_reduced, "easy".to_string());
    print_puzzles(moderate_reduced, "moderate".to_string());
    print_puzzles(hard_reduced, "hard".to_string());
    print_puzzles(insane_reduced, "insane".to_string());

    println!("{} puzzles", rows.len());
}

fn print_puzzles(rows: Vec<(String, u8, u32, f32)>, difficulty: String) {
    for row in rows {
        println!("{{difficulty:'{}', board:'{}'}},", difficulty, row.0);
    }
}

fn sample_balance_board_type(rows: Vec<(String, u8, u32, f32)>, length: u8) -> Vec<(String, u8, u32, f32)> {
    let hex_rows = rows.iter().filter(|x| x.0.contains("~H")).map(|x| x.clone()).collect();
    let rect_rows = rows.iter().filter(|x| x.0.contains("~R")).map(|x| x.clone()).collect();
    let mut rect_sample = sample_by_difficulty(rect_rows, length / 2);
    let mut hex_sample = sample_by_difficulty(hex_rows, length / 2);
    // zip together the samples
    let mut sample = Vec::new();
    for _i in 0..length / 2 {
        if rect_sample.len() > 0 {
            sample.push(rect_sample.remove(0));
        }
        if hex_sample.len() > 0 {
            sample.push(hex_sample.remove(0));
        }
    }
    
    sample
}

fn sample_by_difficulty(rows: Vec<(String, u8, u32, f32)>, length: u8) -> Vec<(String, u8, u32, f32)> {
    let mut rows = rows.clone();
    rows.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    
    if rows.len() < length as usize {
        return rows.to_vec();
    }
    let min_difficulty = rows[0].3;
    let max_difficulty = rows[rows.len() - 1].3;
    let difficulty_span = max_difficulty - min_difficulty;
    let step_size = difficulty_span / ((length) as f32);
    let mut reduced_rows = Vec::new();
    let mut difficulty = min_difficulty;
    for i in 0..rows.len() {
        if rows[i].3 >= difficulty {
            difficulty += step_size;
            reduced_rows.push(rows[i].clone());
        }
        if reduced_rows.len() == length as usize {
            break;
        }
    }
    reduced_rows
}
