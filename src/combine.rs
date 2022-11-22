extern crate rand;
use std::fs;

pub fn combine_puzzles() {
    let directories = vec!["rect", "hex"];

    let mut best_puzzles: Vec<(String, f32, u8, u32)> = Vec::new();

    // loop through the folders in each directory
    for directory in directories {
        let path = format!("{}", directory);
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap();
            // loop through the files in each folder
            let files = fs::read_dir(path).unwrap();
            for file in files {
                let file = file.unwrap().path();
                let file = file.to_str().unwrap();

                // read the file
                let contents = fs::read_to_string(file).unwrap();
                // split the file into lines
                let lines: Vec<&str> = contents.lines().collect();
                let mut rows: Vec<(String, u8, u32, f32)> = Vec::new();
                // loop through the lines
                for line in lines {
                    let columns: Vec<&str> = line.split('\t').collect();
                    let board = columns[0].to_string();
                    let moves = columns[1].parse::<u8>().unwrap();
                    let iterations = columns[2].parse::<u32>().unwrap();
                    let person_count = board.matches('P').count();
                    let cow_count = board.matches('O').count();
                    let mover_count = cow_count + person_count;
                    let difficulty =
                        (moves as f32) * f32::log2(iterations as f32) / (mover_count as f32);
                    rows.push((board, moves, iterations, difficulty));
                }
                // sort the rows by difficulty
                rows.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

                let most_difficult = &rows[rows.len() - 1];
                let mut hex_rect = 'R';
                if directory == "hex" {
                    hex_rect = 'H';
                }
                let puzzle_string =
                    format!("{}~{}{}", most_difficult.1, hex_rect, most_difficult.0);
                best_puzzles.push((
                    puzzle_string,
                    most_difficult.3,
                    most_difficult.1,
                    most_difficult.2,
                ));
            }
        }
    }

    best_puzzles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    // drop duplicate difficulties
    best_puzzles.dedup_by(|a, b| a.1 == b.1);

    random_drop_to_length(&mut best_puzzles, 400);

    // print best puzzles
    for puzzle in &best_puzzles {
        println!("{}\t\t{}\t{}\t{}", puzzle.0, puzzle.1, puzzle.2, puzzle.3);
    }

    println!("{} puzzles", best_puzzles.len());
}

fn random_drop_to_length<T>(vector: &mut Vec<T>, length: usize) {
    let mut i = 0;
    let to_remove = vector.len() - length;
    while i < to_remove {
        let index = (i / to_remove) * vector.len();
        vector.remove(index);
        i += 1;
    }
}
