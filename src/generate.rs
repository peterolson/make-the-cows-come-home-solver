use std::collections::{HashSet, HashMap};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::{cmp};

use crate::board::Board;
use crate::settings::HEXAGONAL_MODE;
use crate::solve::{solve, Solution};

const DIRECTORY: &str = if HEXAGONAL_MODE { "hex" } else { "rect" };

pub fn generate_files(width: u8, height: u8, solution_map : &mut HashMap<Board, Solution>) {
    fs::create_dir_all(format!("{}/{}_{}", DIRECTORY, width, height))
        .expect("Unable to create directory.");
    let length = width * height;

    let mut piece_configurations = Vec::new();
    for barn_count in 1..3 {
        for house_count in 0..2 {
            let max_cow_count = cmp::min(length - barn_count - house_count, 6);
            for cow_count in 1..max_cow_count {
                let max_person_count = cmp::min(length - barn_count - house_count - cow_count, 6);
                for person_count in 0..max_person_count {
                    if cow_count + person_count >= length / 2 {
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

    piece_configurations = vec![(5,3,2,1,4,1)];

    let mut completed = 0;
    let total = piece_configurations.len();
    for piece_configuration in piece_configurations {
        let (width, height, cow_count, barn_count, person_count, house_count) = piece_configuration;
        let file_name = format!(
            "{}/{}_{}/{}_{}_{}_{}_{}_{}_0.txt",
            DIRECTORY,
            width,
            height,
            width,
            height,
            cow_count,
            barn_count,
            person_count,
            house_count
        );
        if Path::new(&file_name).exists() {
            println!("File {} already exists. Continuing...", file_name);
            continue;
        }

        let generated_rows = generate_file(
            width,
            height,
            cow_count,
            barn_count,
            person_count,
            house_count,
            solution_map,
        );

        generate_empty_variations(
            width,
            height,
            cow_count,
            barn_count,
            person_count,
            house_count,
            1,
            generated_rows.clone(),
            solution_map
        );

        completed += 1;
        println!(
            "Completed {}/{}: {}",
            completed,
            total,
            file_name
        );
    }

}

pub fn generate_file(
    width: u8,
    height: u8,
    cow_count: u8,
    barn_count: u8,
    person_count: u8,
    house_count: u8,
    solution_map : &mut HashMap<Board, Solution>
) -> Vec<(String, usize, usize, bool, bool, bool, bool)> {
    let file_name = format!(
        "{}_{}_{}_{}_{}_{}_0.txt",
        width, height, cow_count, barn_count, person_count, house_count
    );
    println!("Generating {}", file_name);
    let mut rows: Vec<(String, usize, usize, bool, bool, bool, bool)> = Vec::new();
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
    let mut traversed_total = 0;

    for board in boards {
        traversed_total += 1;
        if traversed_total % 100000 == 0 {
            println!(
                "Traversed {} boards. {} elegant, {} solvable. {} cached solutions.",
                traversed_total, elegant_total, solvable_total, solution_map.len()
            );
        }
        let serialized = board.to_string();
        let solution = solve(board.clone(), solution_map);
        if solution.can_be_solved{
            solvable_total += 1;
        }
        let is_elegant = solution.is_elegant(&board);
        if is_elegant {
            elegant_total += 1;
        }
        let uses_all_pieces = solution.uses_all_pieces(&board);
        let uses_all_rows_columns = solution.uses_all_rows_columns(&board);
        rows.push((
            serialized,
            solution.move_count as usize,
            solution.tree_size,
            solution.can_be_solved,
            is_elegant,
            uses_all_pieces,
            uses_all_rows_columns,
        ));
    }

    println!("{} solvable, {} elegant", solvable_total, elegant_total);

    sort_rows(&mut rows);

    let lines = rows
        .iter()
        .filter(|x| x.4)
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

fn sort_rows(rows: &mut Vec<(String, usize, usize, bool, bool, bool, bool)>) {
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
        if a.6 && !b.6 {
            return std::cmp::Ordering::Less;
        }
        if !a.6 && b.6 {
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
    rows: Vec<(String, usize, usize, bool, bool, bool, bool)>,
    solution_map : &mut HashMap<Board, Solution>
) -> Vec<(String, usize, usize, bool, bool, bool, bool)> {
    let mut rows_with_empty: Vec<(String, usize, usize, bool, bool, bool, bool)> = Vec::new();
    let file_name = format!(
        "{}_{}_{}_{}_{}_{}_{}.txt",
        width, height, cow_count, barn_count, person_count, house_count, empty_count
    );
    println!("Generating {}", file_name);
    let mut elegant_count = 0;
    let mut traversed_count = 0;
    let rows_len = rows.len();
    let mut encountered_boards: HashSet<Board> = HashSet::new();
    for row in rows {
        traversed_count += 1;
        if traversed_count % 10000 == 0 {
            println!(
                "Traversed {} boards of {}. {} elegant. {} cached solutions.",
                traversed_count, rows_len, elegant_count, solution_map.len()
            );
        }
        let variations = get_empty_variations(row, solution_map);
        for variation in variations {
            let board = Board::from_string(&variation.0);
            if encountered_boards.contains(&board) {
                continue;
            }
            if variation.4 {
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
        .filter(|x| x.4)
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
    row: (String, usize, usize, bool, bool, bool, bool),
    solution_map : &mut HashMap<Board, Solution>
) -> Vec<(String, usize, usize, bool, bool, bool, bool)> {
    let mut rows: Vec<(String, usize, usize, bool, bool, bool, bool)> = Vec::new();
    let (serialized, moves, _iterations, can_be_solved, _is_elegant, _uses_all_pieces, uses_all_rows_columns) = row;
    if !can_be_solved {
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
        let solution = solve(new_board.clone(), solution_map);
        let is_elegant = solution.is_elegant(&new_board) && (solution.move_count as usize) > moves;
        rows.push((
            new_serialized,
            solution.move_count as usize,
            solution.tree_size,
            solution.can_be_solved,
            is_elegant,
            solution.uses_all_pieces(&new_board),
            solution.uses_all_rows_columns(&new_board),
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
    println!("Generating boards for {}x{}", width, height);
    let strings = generate_with_prefix(
        length,
        "",
        cow_count,
        barn_count,
        person_count,
        house_count,
        empty_count,
    );
    println!("{} boards generated", strings.len());
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
    println!("{} unique boards generated", boards.len());
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
