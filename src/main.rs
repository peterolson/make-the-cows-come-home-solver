mod board;
mod combine;
mod generate;
mod reverse_solver;
mod settings;
mod solve;

fn main() {
    println!("Hello, world!");

    //let mut solution_map = HashMap::new();

    // //combine_puzzles();
    // generate_files(3, 2);
    // generate_files(4, 2);
    // generate_files(3,3);
    // generate_files(4, 3);
    // generate_files(5, 3, &mut solution_map);

    // let board = Board::from_string("5|3|PO__P____HP_OBP");

    // let solution = solve(board, &mut solution_map);

    // println!("Solution: {:?} {}", solution.moves, solution.moves.len());

    // let board = Board::from_string("4|3|O_PBB__HPO_P");
    // let solution = solve(board, &mut solution_map);

    // let board = Board::from_string("5|3|PO_OPHP____B_P_");
    // let solution = solve(board, &mut solution_map);

    let board = board::Board::from_string("4|4|________B______H");

    let reverse_solution = reverse_solver::reverse_solve(board, 5, 3);

    println!("{}", reverse_solution.0);
    println!(
        "Solution: {:?} {}",
        reverse_solution.1.moves,
        reverse_solution.1.moves.len()
    );

    print!("Done");
}
