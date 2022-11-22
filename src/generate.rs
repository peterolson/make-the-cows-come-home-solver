use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{cmp, thread};

use crate::board::Board;
use crate::settings::HEXAGONAL_MODE;
use crate::solve::solve;

const DIRECTORY: &str = if HEXAGONAL_MODE { "hex" } else { "rect" };

pub fn generate_files(width: u8, height: u8) {
    fs::create_dir_all(format!("{}/{}_{}", DIRECTORY, width, height))
        .expect("Unable to create directory.");
    let length = width * height;

    let mut handles = vec![];
    let mut piece_configurations = Vec::new();
    for barn_count in 1..3 {
        for house_count in 0..2 {
            let max_cow_count = cmp::min(length - barn_count - house_count, 6);
            for cow_count in 1..max_cow_count {
                let max_person_count = cmp::min(length - barn_count - house_count - cow_count, 6);
                for person_count in 0..max_person_count {
                    if barn_count + house_count + cow_count + person_count > length / 2 {
                        continue;
                    }
                    piece_configurations.push((
                        width,
                        height,
                        cow_count,
                        barn_count,
                        person_count,
                        house_count,
                    ));
                }
            }
        }
    }

    let total_configurations = piece_configurations.len();
    let completed = Arc::new(Mutex::new(0));
    let current_index = Arc::new(Mutex::new(0));
    const MAX_CONCURRENT_THREADS: i8 = 8;

    for thread_number in 0..MAX_CONCURRENT_THREADS {
        let current_index = Arc::clone(&current_index);
        let completed = Arc::clone(&completed);
        let piece_configurations = piece_configurations.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} started.", thread_number);
            loop {
                let i;
                {
                    let mut index = current_index.lock().unwrap();
                    i = *index;
                    if i >= total_configurations {
                        break;
                    }
                    *index += 1;
                }
                let (width, height, cow_count, barn_count, person_count, house_count) =
                    piece_configurations[i];

                let generated_rows = generate_file(
                    width,
                    height,
                    cow_count,
                    barn_count,
                    person_count,
                    house_count,
                );
                let mut seed_rows = generated_rows.clone();

                let max_empty_count = 0; // length - barn_count - house_count - cow_count - person_count;
                for empty_count in 1..max_empty_count {
                    let empty_variations = generate_empty_variations(
                        width,
                        height,
                        cow_count,
                        barn_count,
                        person_count,
                        house_count,
                        empty_count,
                        seed_rows,
                    );
                    seed_rows = empty_variations.clone();
                    if empty_variations.len() == 0 {
                        break;
                    }
                }
                {
                    let mut completed = completed.lock().unwrap();
                    *completed += 1;
                    println!(
                        "Thread {}: Completed configuration #{}. Finished {} of {}.",
                        thread_number, i, *completed, total_configurations
                    );
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn generate_file(
    width: u8,
    height: u8,
    cow_count: u8,
    barn_count: u8,
    person_count: u8,
    house_count: u8,
) -> Vec<(String, usize, isize, bool, bool, bool)> {
    let file_name = format!(
        "{}_{}_{}_{}_{}_{}_0.txt",
        width, height, cow_count, barn_count, person_count, house_count
    );
    println!("Generating {}", file_name);
    let mut rows: Vec<(String, usize, isize, bool, bool, bool)> = Vec::new();
    let boards = generate_boards(
        width,
        height,
        cow_count,
        barn_count,
        person_count,
        house_count,
        0,
    );

    println!("{} boards found", boards.len());

    let mut elegant_total = 0;
    let mut solvable_total = 0;

    for board in boards {
        let serialized = board.to_string();
        let (situation, iterations) = solve(board);
        if iterations > 0 {
            solvable_total += 1;
        }
        let is_elegant = situation.is_elegant();
        if is_elegant {
            elegant_total += 1;
        }
        let uses_all_pieces = situation.uses_all_pieces();
        let uses_all_rows_columns = situation.uses_all_rows_columns();
        rows.push((
            serialized,
            situation.moves.len(),
            iterations,
            is_elegant,
            uses_all_pieces,
            uses_all_rows_columns,
        ));
    }

    println!("{} solvable, {} elegant", solvable_total, elegant_total);

    sort_rows(&mut rows);

    let lines = rows
        .iter()
        .filter(|x| x.3)
        .map(|row| format!("{}\t{}\t{}", row.0, row.1, row.2))
        .collect::<Vec<String>>();
    if lines.len() == 0 {
        return rows;
    }

    let data = lines.join("\n");
    let mut f = File::create(format!("{}/{}_{}/{}", DIRECTORY, width, height, file_name))
        .expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
    rows
}

fn sort_rows(rows: &mut Vec<(String, usize, isize, bool, bool, bool)>) {
    rows.sort_by(|a, b| {
        if a.3 && !b.3 {
            return std::cmp::Ordering::Less;
        }
        if !a.3 && b.3 {
            return std::cmp::Ordering::Greater;
        }
        if a.4 && !b.4 {
            return std::cmp::Ordering::Less;
        }
        if !a.4 && b.4 {
            return std::cmp::Ordering::Greater;
        }
        if a.5 && !b.5 {
            return std::cmp::Ordering::Less;
        }
        if !a.5 && b.5 {
            return std::cmp::Ordering::Greater;
        }
        if a.2 < b.2 {
            return std::cmp::Ordering::Greater;
        }
        if a.2 > b.2 {
            return std::cmp::Ordering::Less;
        }
        if a.1 < b.1 {
            return std::cmp::Ordering::Greater;
        }
        if a.1 > b.1 {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    });
}

pub fn generate_empty_variations(
    width: u8,
    height: u8,
    cow_count: u8,
    barn_count: u8,
    person_count: u8,
    house_count: u8,
    empty_count: u8,
    rows: Vec<(String, usize, isize, bool, bool, bool)>,
) -> Vec<(String, usize, isize, bool, bool, bool)> {
    let mut rows_with_empty: Vec<(String, usize, isize, bool, bool, bool)> = Vec::new();
    let file_name = format!(
        "{}_{}_{}_{}_{}_{}_{}.txt",
        width, height, cow_count, barn_count, person_count, house_count, empty_count
    );
    println!("Generating {}", file_name);
    let mut elegant_count = 0;
    let mut encountered_boards: HashSet<Board> = HashSet::new();
    for row in rows {
        let variations = get_empty_variations(row);
        for variation in variations {
            let board = Board::from_string(&variation.0);
            if encountered_boards.contains(&board) {
                continue;
            }
            if variation.3 {
                elegant_count += 1;
            }
            rows_with_empty.push(variation);
            let symmetric_variants = board.get_symmetric_variants();
            encountered_boards.insert(board);
            for symmetric_variant in symmetric_variants {
                encountered_boards.insert(symmetric_variant);
            }
        }
    }

    if elegant_count == 0 {
        println!("No elegant solutions with empty space.");
        return Vec::new();
    }

    println!(
        "{} rows found, {} elegant",
        rows_with_empty.len(),
        elegant_count
    );

    sort_rows(&mut rows_with_empty);

    if rows_with_empty.len() == 0 {
        return rows_with_empty;
    }

    let lines = rows_with_empty
        .iter()
        .filter(|x| x.3)
        .map(|row| format!("{}\t{}\t{}", row.0, row.1, row.2))
        .collect::<Vec<String>>();
    if lines.len() == 0 {
        return rows_with_empty;
    }

    let data = lines.join("\n");
    let mut f = File::create(format!("{}/{}_{}/{}", DIRECTORY, width, height, file_name))
        .expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
    return rows_with_empty;
}

pub fn get_empty_variations(
    row: (String, usize, isize, bool, bool, bool),
) -> Vec<(String, usize, isize, bool, bool, bool)> {
    let mut rows: Vec<(String, usize, isize, bool, bool, bool)> = Vec::new();
    let (serialized, moves, iterations, _is_elegant, _uses_all_pieces, uses_all_rows_columns) = row;
    if iterations < 0 {
        return rows;
    }
    if !uses_all_rows_columns {
        return rows;
    }
    for i in 0..serialized.len() {
        let ch = serialized.chars().nth(i).unwrap();
        if ch != '_' {
            continue;
        }
        let new_serialized = format!("{}{}{}", &serialized[0..i], 'E', &serialized[i + 1..]);
        let new_board = Board::from_string(new_serialized.as_str());
        let (situation, new_iterations) = solve(new_board);
        let is_elegant = situation.is_elegant() && situation.moves.len() > moves;
        rows.push((
            new_serialized,
            situation.moves.len(),
            new_iterations,
            is_elegant,
            situation.uses_all_pieces(),
            situation.uses_all_rows_columns(),
        ));
    }
    rows
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
