mod board;

fn main() {
    println!("Hello, world!");

    let board = board::Board::from_string("4|4|_O_PB______OPO_P");
    let possible_moves = board.get_possible_moves();
    // print all possible moves
    for (i, b) in possible_moves.iter().enumerate() {
        println!("Move {}:", i + 1);
        println!("{}", b.to_string());
    }
    println!("Board: {:?}", board.to_string());
}

