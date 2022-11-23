use std::fmt;

use crate::settings::HEXAGONAL_MODE;

const MAX_BOARD_SIZE : usize = 36;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Piece {
    Cow,
    Person,
    House,
    Barn,
    Empty,
    Blank,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Board {
    pub width: u8,
    pub height: u8,
    pub pieces: [Piece; MAX_BOARD_SIZE]
}

impl Board {
    fn new(width: u8, height: u8) -> Board {
        Board {
            width: width,
            height: height,
            pieces: [Piece::Blank; MAX_BOARD_SIZE],
        }
    }

    fn get(&self, x: u8, y: u8) -> Piece {
        self.pieces[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: u8, y: u8, piece: Piece) {
        self.pieces[(y * self.width + x) as usize] = piece;
    }

    fn set_index(&mut self, index: usize, piece: Piece) {
        self.pieces[index] = piece;
    }

    pub fn get_moves_from(&self, index: u8) -> Vec<(u8, u8)> {
        let mut moves: Vec<(u8, u8)> = Vec::new();
        let x = index % self.width;
        let y = index / self.width;
        let piece = self.get(x, y);

        // left
        let mut new_x = x;
        let offset = y * self.width;
        while new_x > 0 {
            new_x -= 1;
            if check_move(
                piece,
                self.get(new_x, y),
                &mut moves,
                offset + new_x,
                offset + new_x + 1,
                x - new_x > 1,
            ) {
                break;
            }
        }
        // right
        new_x = x;
        while new_x < self.width - 1 {
            new_x += 1;
            if check_move(
                piece,
                self.get(new_x, y),
                &mut moves,
                offset + new_x,
                offset + new_x - 1,
                new_x - x > 1,
            ) {
                break;
            }
        }
        if HEXAGONAL_MODE {
            // up left
            let mut new_x = x;
            let mut new_y = y;
            while new_x + (new_y % 2) > 0 && new_y > 0 {
                new_y -= 1;
                new_x -= new_y % 2;
                if check_move(
                    piece,
                    self.get(new_x, new_y),
                    &mut moves,
                    (new_y * self.width) + new_x,
                    ((new_y + 1) * self.width) + new_x + new_y % 2,
                    y - new_y > 1,
                ) {
                    break;
                }
            }
            // up right
            new_x = x;
            new_y = y;
            while new_x < self.width - (new_y % 2) && new_y > 0 {
                new_y -= 1;
                new_x += (new_y + 1) % 2;
                if check_move(
                    piece,
                    self.get(new_x, new_y),
                    &mut moves,
                    (new_y * self.width) + new_x,
                    ((new_y + 1) * self.width) + new_x - (new_y + 1) % 2,
                    y - new_y > 1,
                ) {
                    break;
                }
            }

            // down left
            new_x = x;
            new_y = y;
            while new_x + (new_y % 2) > 0 && new_y < self.height - 1 {
                new_y += 1;
                new_x -= new_y % 2;
                if check_move(
                    piece,
                    self.get(new_x, new_y),
                    &mut moves,
                    (new_y * self.width) + new_x,
                    ((new_y - 1) * self.width) + new_x + new_y % 2,
                    new_y - y > 1,
                ) {
                    break;
                }
            }

            // down right
            new_x = x;
            new_y = y;
            while new_x < self.width - (new_y % 2) && new_y < self.height - 1 {
                new_y += 1;
                new_x += (new_y + 1) % 2;
                if check_move(
                    piece,
                    self.get(new_x, new_y),
                    &mut moves,
                    (new_y * self.width) + new_x,
                    ((new_y - 1) * self.width) + new_x - (new_y + 1) % 2,
                    new_y - y > 1,
                ) {
                    break;
                }
            }
        } else {
            // up
            let mut new_y = y;
            while new_y > 0 {
                new_y -= 1;
                if check_move(
                    piece,
                    self.get(x, new_y),
                    &mut moves,
                    (new_y * self.width) + x,
                    ((new_y + 1) * self.width) + x,
                    y - new_y > 1,
                ) {
                    break;
                }
            }
            // down
            new_y = y;
            while new_y < self.height - 1 {
                new_y += 1;
                if check_move(
                    piece,
                    self.get(x, new_y),
                    &mut moves,
                    (new_y * self.width) + x,
                    ((new_y - 1) * self.width) + x,
                    new_y - y > 1,
                ) {
                    break;
                }
            }
        }

        moves
    }

    pub fn get_possible_moves(&self) -> Vec<(Board, u8, u8, u8)> {
        let mut moves: Vec<(Board, u8, u8, u8)> = Vec::new();
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];
            if piece == Piece::Cow || piece == Piece::Person {
                let destinations = self.get_moves_from(i as u8);
                for (destination, puller) in destinations {
                    let mut new_board = self.clone();
                    let piece = new_board.pieces[i];
                    let destination_piece = new_board.pieces[destination as usize];
                    new_board.set_index(i, Piece::Blank);
                    if destination_piece == Piece::Blank {
                        new_board.set_index(destination as usize, piece);
                    }
                    moves.push((new_board, i as u8, destination, puller));
                }
            }
        }
        moves
    }

    pub fn is_solved(&self) -> bool {
        let mut person_count = 0;
        let mut cow_count = 0;
        let mut house_count = 0;
        let mut barn_count = 0;
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];
            match piece {
                Piece::Person => person_count += 1,
                Piece::Cow => cow_count += 1,
                Piece::House => house_count += 1,
                Piece::Barn => barn_count += 1,
                _ => {}
            }
        }
        person_count * house_count + cow_count * barn_count == 0
    }

    pub fn from_string(s: &str) -> Board {
        // string of format "width|height|pieces"
        let mut parts = s.split('|');
        let width = parts.next().unwrap().parse::<u8>().unwrap();
        let height = parts.next().unwrap().parse::<u8>().unwrap();
        let mut board = Board::new(width, height);
        let mut x = 0;
        let mut y = 0;
        for c in parts.next().unwrap().chars() {
            let piece = match c {
                'O' => Piece::Cow,
                'P' => Piece::Person,
                'H' => Piece::House,
                'B' => Piece::Barn,
                'E' => Piece::Empty,
                '_' => Piece::Blank,
                _ => panic!("Invalid character in board string"),
            };
            board.set(x, y, piece);
            x += 1;
            if x == width {
                x = 0;
                y += 1;
            }
        }
        board
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.width.to_string());
        s.push('|');
        s.push_str(&self.height.to_string());
        s.push('|');
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    Piece::Cow => 'O',
                    Piece::Person => 'P',
                    Piece::House => 'H',
                    Piece::Barn => 'B',
                    Piece::Empty => 'E',
                    Piece::Blank => '_',
                };
                s.push(c);
            }
        }
        s
    }

    fn rotate(&mut self) {
        let mut new_pieces = [Piece::Blank; MAX_BOARD_SIZE];
        for y in 0..self.height {
            for x in 0..self.width {
                let piece = self.get(x, y);
                new_pieces[(x * self.height + (self.height - y - 1)) as usize] = piece;
            }
        }
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
        self.pieces = new_pieces;
    }

    fn flip_horizontal(&mut self) {
        let mut new_pieces = [Piece::Blank; MAX_BOARD_SIZE];
        let width = self.width;
        for y in 0..self.height {
            for x in 0..self.width {
                let piece = self.get(x, y);
                new_pieces[(y * width + (width - x - 1)) as usize] = piece;
            }
        }
        self.pieces = new_pieces;
    }

    fn switch_cow_person(&mut self) {
        for i in 0..self.pieces.len() {
            let piece = self.pieces[i];
            match piece {
                Piece::Cow => self.pieces[i] = Piece::Person,
                Piece::Person => self.pieces[i] = Piece::Cow,
                Piece::House => self.pieces[i] = Piece::Barn,
                Piece::Barn => self.pieces[i] = Piece::House,
                _ => {}
            }
        }
    }

    pub fn get_symmetric_variants(&self) -> Vec<Board> {
        let mut variants = Vec::new();
        let mut board = self.clone();
        variants.push(board.clone());
        for _ in 0..3 {
            board.rotate();
            variants.push(board.clone());
        }
        board.flip_horizontal();
        variants.push(board.clone());
        for _ in 0..3 {
            board.rotate();
            variants.push(board.clone());
        }
        let count = variants.len();
        for i in 0..count {
            let mut board = variants[i].clone();
            board.switch_cow_person();
            variants.push(board);
        }
        variants
    }
}

fn check_move(
    piece: Piece,
    new_piece: Piece,
    moves: &mut Vec<(u8, u8)>,
    space: u8,
    prev_space: u8,
    has_prev_space: bool,
) -> bool {
    if new_piece == Piece::Empty {
        return true;
    }
    if new_piece == Piece::Person
        || new_piece == Piece::Cow
        || (new_piece == Piece::House && piece == Piece::Cow)
        || (new_piece == Piece::Barn && piece == Piece::Person)
    {
        if has_prev_space {
            moves.push((prev_space, space));
        }
        return true;
    }
    if (new_piece == Piece::House && piece == Piece::Person)
        || (new_piece == Piece::Barn && piece == Piece::Cow)
    {
        moves.push((space, space));
        return true;
    }
    return false;
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    Piece::Cow => 'O',
                    Piece::Person => 'P',
                    Piece::House => 'H',
                    Piece::Barn => 'B',
                    Piece::Empty => 'E',
                    Piece::Blank => '_',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
