mod board;
mod solve;

fn main() {
    println!("Hello, world!");

    let board = board::Board::from_string("5|6|__O__O___OO___B__O__O___PO___O");
    let possible_moves = board.get_possible_moves();
    // print all possible moves
    for (i, b) in possible_moves.iter().enumerate() {
        println!("Move {}:", i + 1);
        println!("{} {}", b.to_string(), b.is_solved());
    }
    println!("Board: {:?} {}", board.to_string(), board.is_solved());

    let (moves, iterations) = solve::solve(board);
    println!("Solved in {} moves in {} iterations", moves, iterations);
}

