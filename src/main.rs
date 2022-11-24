mod board;
mod combine;
mod generate;
mod reverse_solver;
mod settings;
mod solve;

fn main() {
    println!("Hello, world!");

    generate::generate_puzzles(3,3);

    // let board = board::Board::from_string("6|4|_BE______E__H___");

    // let reverse_solution = reverse_solver::reverse_solve(board, 5, 3);

    // println!("{}", reverse_solution.0);
    // print!("{}", reverse_solution.0.to_string());
    // println!(
    //     "Solution: {:?} {}",
    //     reverse_solution.1.moves,
    //     reverse_solution.1.moves.len()
    // );

    print!("Done");
}
