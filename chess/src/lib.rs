pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::cmp;

#[derive(PartialEq, Eq)]
pub enum Piece {
    King, 
    Queen, 
    Rook, 
    Bishop, 
    Knight, 
    Pawn, 
    None
}

#[derive(PartialEq, Eq)]
pub enum Color {
    White, 
    Black, 
    None
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square {
    pub row: i32, 
    pub column: i32
}

impl Square {
    pub fn new(row: i32, column: i32) -> Square {
        Square { row, column }
    }

    pub fn to_string(&self) -> String {
        ("ABCDEFGH".as_bytes()[self.column as usize] as char).to_string() + &(self.row + 1).to_string()
    }

    pub fn to_i32(&self) -> i32 {
        self.row * 8 + self.column
    }

    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.row <= 7 && self.column >= 0 && self.column <= 7
    }
}

pub fn square_from_string(value: String) -> Square {
    let row: i32 = (value.as_bytes()[1] as char).to_string().parse::<i32>().unwrap() - 1;
    let column: i32 = "ABCDEFGH".chars().position(|c| c == value.as_bytes()[0] as char).unwrap() as i32;
    Square { row, column }
}

pub fn square_from_i32(value: i32) -> Square {
    Square { row: value / 8, column: value % 8 }
}

fn is_bit(value: u64, bit: i32) -> bool {
    ((value >> bit) & 1) == 1
}


#[derive(Copy, Clone)]
pub struct BitBoard {
    pub value: u64
}

impl BitBoard {
    pub fn new(value: u64) -> BitBoard {
        BitBoard { value }
    }

    pub fn get_square(&self, square: Square) -> bool {
        if square.is_valid() {
            return self.value >> square.row * 8 + square.column & 1 == 1;
        }
        false
    }

    pub fn set_square(&mut self, square: Square) {
        self.value = self.value | (1 << square.to_i32());
    }

    pub fn remove_square(&mut self, square: Square) {
        self.value = self.value & (!(1 << square.to_i32()));
    }

    pub fn to_squares(&self) -> Vec<Square> {
        let mut vec: Vec<Square> = Vec::new();
        for i in 0i32..64 {
            if is_bit(self.value, i) {
                let square: Square = square_from_i32(i);
                vec.push(square);
            }
        }
        vec
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square
}

impl Move {
    pub fn new(from: Square, to: Square) -> Move {
        Move { from, to }
    }
}

#[derive(Copy, Clone)]
pub struct ChessBoard {
    pub white_pawn: BitBoard,
    pub white_rook: BitBoard,
    pub white_knight: BitBoard,
    pub white_bishop: BitBoard,
    pub white_queen: BitBoard,
    pub white_king: BitBoard,
    pub black_pawn: BitBoard,
    pub black_rook: BitBoard,
    pub black_knight: BitBoard,
    pub black_bishop: BitBoard,
    pub black_queen: BitBoard,
    pub black_king: BitBoard,
    pub moved_pieces: BitBoard,
    pub white_turn: bool,
}

impl ChessBoard {

    pub fn new() -> ChessBoard {
        ChessBoard { 
            white_pawn: BitBoard::new((1 << 8) + (1 << 9) + (1 << 10) + (1 << 11) + (1 << 12) + (1 << 13) + (1 << 14) + (1 << 15)), 
            white_rook: BitBoard::new((1 << 0) + (1 << 7)), 
            white_knight: BitBoard::new((1 << 1) + (1 << 6)),
            white_bishop: BitBoard::new((1 << 2) + (1 << 5)),
            white_queen: BitBoard::new(1 << 3), 
            white_king: BitBoard::new(1 << 4),  
            black_pawn: BitBoard::new((1 << 48) + (1 << 49) + (1 << 50) + (1 << 51) + (1 << 52) + (1 << 53) + (1 << 54) + (1 << 55)), 
            black_rook: BitBoard::new((1 << 56) + (1 << 63)), 
            black_knight: BitBoard::new((1 << 57) + (1 << 62)),  
            black_bishop: BitBoard::new((1 << 58) + (1 << 61)), 
            black_queen: BitBoard::new(1 << 59),  
            black_king: BitBoard::new(1 << 60), 
            moved_pieces: BitBoard::new(0),
            white_turn: true, }
    }

    pub fn new_fen(fen: String) -> ChessBoard {
        let mut board: ChessBoard = ChessBoard::new();



        board
    }

    pub fn print(&self) {
        println!("\n   --------------------------------");
        for r in 0..8 {
                print!("{} |", 8 - r);
                for c in 0..8 {
                let square: Square = Square::new(7 - r, c);
                if self.white_pawn.get_square(square) { print!(" P "); } 
                else if self.black_pawn.get_square(square) { print!(" p "); } 
                else if self.white_rook.get_square(square) { print!(" R "); } 
                else if self.black_rook.get_square(square) { print!(" r "); } 
                else if self.white_bishop.get_square(square) { print!(" B "); } 
                else if self.black_bishop.get_square(square) { print!(" b "); } 
                else if self.white_knight.get_square(square) { print!(" N "); } 
                else if self.black_knight.get_square(square) { print!(" n "); } 
                else if self.white_queen.get_square(square) { print!(" Q "); } 
                else if self.black_queen.get_square(square) { print!(" q "); } 
                else if self.white_king.get_square(square) { print!(" K "); } 
                else if self.black_king.get_square(square) { print!(" k "); } 
                else { print!("   "); }
                print!("|");
            }
            println!("\n   --------------------------------");
        }
        println!("    A   B   C   D   E   F   G   H")
    }

    pub fn white_pieces(&self) -> BitBoard {
        BitBoard::new(
        self.white_pawn.value   | self.white_rook.value   | 
        self.white_knight.value | self.white_bishop.value | 
        self.white_queen.value  | self.white_king.value)
    }

    pub fn black_pieces(&self) -> BitBoard {
        BitBoard::new(
        self.black_pawn.value   | self.black_rook.value   | 
        self.black_knight.value | self.black_bishop.value | 
        self.black_queen.value  | self.black_king.value)
    }

    pub fn empty_squares(&self) -> BitBoard {
        BitBoard::new(
            !(self.white_pieces().value | self.black_pieces().value)
        )
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();   

        if self.white_turn {
            vec.extend(self.generate_white_pawn_moves());
            vec.extend(self.generate_white_rook_moves());
            vec.extend(self.generate_white_knight_moves());
            vec.extend(self.generate_white_bishop_moves());
            vec.extend(self.generate_white_queen_moves());
            vec.extend(self.generate_white_king_moves());
        }
        else {
            vec.extend(self.generate_black_pawn_moves());
            vec.extend(self.generate_black_rook_moves());
            vec.extend(self.generate_black_knight_moves());
            vec.extend(self.generate_black_bishop_moves());
            vec.extend(self.generate_black_queen_moves());
            vec.extend(self.generate_black_king_moves());
        }

        vec.into_iter().filter(|x| self.is_move_valid(x)).collect::<Vec<Move>>() as Vec<Move>
    }

    pub fn is_move_valid(&self, m: &Move) -> bool {
        if m.from == square_from_string("E1".to_string()) {
            if m.to == square_from_string("C1".to_string()) && 
            !self.is_move_valid(&Move::new(
                square_from_string("E1".to_string()), 
                square_from_string("D1".to_string()))
            ) {
                return false;
            }
            if m.to == square_from_string("G1".to_string()) && 
            !self.is_move_valid(&Move::new(
                square_from_string("E1".to_string()), 
                square_from_string("F1".to_string()))
            ) {
                return false;
            }
        }
        if m.from == square_from_string("E8".to_string()) {
            if m.to == square_from_string("C8".to_string()) && 
            !self.is_move_valid(&Move::new(
                square_from_string("E8".to_string()), 
                square_from_string("D8".to_string()))
            ) {
                return false;
            }
            if m.to == square_from_string("G8".to_string()) && 
            !self.is_move_valid(&Move::new(
                square_from_string("E8".to_string()), 
                square_from_string("F8".to_string()))
            ) {
                return false;
            }
        }
        let mut clone: ChessBoard = self.clone();
        clone.make_move(*m, false);
        (clone.white_turn && !clone.is_black_checked()) || (!clone.white_turn && !clone.is_white_checked())
    }

    pub fn generate_white_pawn_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_pawn.to_squares() {
            if from.column < 7 && is_bit(self.black_pieces().value, from.to_i32() + 9) {
                vec.push(Move::new(from, Square::new(from.row+1, from.column+1)));
            }
            if from.column > 0 && is_bit(self.black_pieces().value, from.to_i32() + 7) {
                vec.push(Move::new(from, Square::new(from.row+1, from.column-1)));
            }
            if from.row == 4 {
                if from.column > 0 && self.black_pawn.get_square(square_from_i32(from.to_i32() - 1)) &&
                !self.moved_pieces.get_square(square_from_i32(from.to_i32() + 7)) {
                    vec.push(Move::new(from, Square::new(from.row+1, from.column-1)));
                }
                if from.column < 7 && self.black_pawn.get_square(square_from_i32(from.to_i32() + 1)) &&
                !self.moved_pieces.get_square(square_from_i32(from.to_i32() + 7)) {
                    vec.push(Move::new(from, Square::new(from.row+1, from.column+1)));
                }
            }

            if is_bit(self.empty_squares().value, from.to_i32() + 8) {
                vec.push(Move::new(from, Square::new(from.row+1, from.column)));
            } else { continue; }
            if from.row == 1 && is_bit(self.empty_squares().value, from.to_i32() + 16) {
                vec.push(Move::new(from, Square::new(from.row+2, from.column)));
            }
        }
        vec
    }

    pub fn generate_white_pawn_threats(&self) -> Vec<Square> {
        let mut vec: Vec<Square> = Vec::new();

        for from in self.white_pawn.to_squares() {
            if from.column < 7 {
                vec.push(Square::new(from.row+1, from.column+1));
            }
            if from.column > 0 {
                vec.push(Square::new(from.row+1, from.column-1));
            } 
        }

        vec
    }

    pub fn generate_white_knight_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_knight.to_squares() {
            {
                let to: Square = Square::new(from.row + 2, from.column + 1);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 2, from.column - 1);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 1, from.column + 2);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 1, from.column - 2);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 1, from.column + 2);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 1, from.column - 2);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 2, from.column + 1);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 2, from.column - 1);
                if to.is_valid() && !self.white_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
        }
        vec
    }

    pub fn generate_white_king_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_king.to_squares() {
            for r in -1i32..2 {
                for c in -1i32..2 {
                    if r == 0 && c == 0 { continue; }
                    let to: Square = Square::new(from.row + r, from.column + c);
                    if self.white_pieces().get_square(to) { continue; }
                    if !to.is_valid() { continue; }
                    vec.push(Move::new(from, to));
                }
            }
        }

        if  !self.moved_pieces.get_square(square_from_string("E1".to_string())) &&
            !self.moved_pieces.get_square(square_from_string("H1".to_string())) &&
            self.empty_squares().get_square(square_from_string("F1".to_string())) &&
            self.empty_squares().get_square(square_from_string("G1".to_string())) {
                vec.push(Move::new(square_from_string("E1".to_string()), square_from_string("G1".to_string())));
            }

        if  !self.moved_pieces.get_square(square_from_string("E1".to_string())) &&
            !self.moved_pieces.get_square(square_from_string("A1".to_string())) &&
            self.empty_squares().get_square(square_from_string("B1".to_string())) &&
            self.empty_squares().get_square(square_from_string("C1".to_string())) &&
            self.empty_squares().get_square(square_from_string("D1".to_string())) {
                vec.push(Move::new(square_from_string("E1".to_string()), square_from_string("C1".to_string())));
            }
        
        vec
    }

    pub fn generate_white_rook_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_rook.to_squares() {
            for r in 1..(8-from.row) {
                let to: Square = Square::new(from.row+r, from.column);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for r in 1..(from.row + 1) {
                let to: Square = Square::new(from.row-r, from.column);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for c in 1..(8-from.column) {
                let to: Square = Square::new(from.row, from.column + c);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for c in 1..(from.column + 1) {
                let to: Square = Square::new(from.row, from.column - c);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn generate_white_bishop_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_bishop.to_squares() {
            for i in 1..(8 - cmp::max(from.row, from.column)) {
                let to: Square = Square::new(from.row + i, from.column + i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row, from.column) + 1) {
                let to: Square = Square::new(from.row - i, from.column - i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row + 1, 8 - from.column)) {
                let to: Square = Square::new(from.row - i, from.column + i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(8 - from.row, from.column + 1)) {
                let to: Square = Square::new(from.row + i, from.column - i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn generate_white_queen_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.white_queen.to_squares() {
            for r in 1..(8-from.row) {
                let to: Square = Square::new(from.row+r, from.column);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for r in 1..(from.row + 1) {
                let to: Square = Square::new(from.row-r, from.column);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for c in 1..(8-from.column) {
                let to: Square = Square::new(from.row, from.column + c);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for c in 1..(from.column + 1) {
                let to: Square = Square::new(from.row, from.column - c);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(8 - cmp::max(from.row, from.column)) {
                let to: Square = Square::new(from.row + i, from.column + i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row, from.column) + 1) {
                let to: Square = Square::new(from.row - i, from.column - i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row + 1, 8 - from.column)) {
                let to: Square = Square::new(from.row - i, from.column + i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(8 - from.row, from.column + 1)) {
                let to: Square = Square::new(from.row + i, from.column - i);
                if self.white_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.black_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn generate_black_pawn_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_pawn.to_squares() {
            if from.column < 7 && is_bit(self.white_pieces().value, from.to_i32() - 9) {
                vec.push(Move::new(from, Square::new(from.row-1, from.column-1)));
            }
            if from.column > 0 && is_bit(self.white_pieces().value, from.to_i32() - 7) {
                vec.push(Move::new(from, Square::new(from.row-1, from.column+1)));
            }
            if from.row == 3 {
                if from.column > 0 && self.white_pawn.get_square(square_from_i32(from.to_i32() - 1)) &&
                !self.moved_pieces.get_square(square_from_i32(from.to_i32() - 9)) {
                    vec.push(Move::new(from, Square::new(from.row-1, from.column-1)));
                }
                if from.column < 7 && self.white_pawn.get_square(square_from_i32(from.to_i32() + 1)) &&
                !self.moved_pieces.get_square(square_from_i32(from.to_i32() - 7)) {
                    vec.push(Move::new(from, Square::new(from.row-1, from.column+1)));
                }
            }
            if is_bit(self.empty_squares().value, from.to_i32() - 8) {
                vec.push(Move::new(from, Square::new(from.row-1, from.column)));
            } else { continue; }
            if from.row == 6 && is_bit(self.empty_squares().value, from.to_i32() - 16) {
                vec.push(Move::new(from, Square::new(from.row-2, from.column)));
            }
        }
        vec
    }

    pub fn generate_black_pawn_threats(&self) -> Vec<Square> {
        let mut vec: Vec<Square> = Vec::new();

        for from in self.black_pawn.to_squares() {
            if from.column < 7 {
                vec.push(Square::new(from.row-1, from.column+1));
            }
            if from.column > 0 {
                vec.push(Square::new(from.row-1, from.column-1));
            } 
        }

        vec
    }

    pub fn generate_black_knight_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_knight.to_squares() {
            {
                let to: Square = Square::new(from.row + 2, from.column + 1);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 2, from.column - 1);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 1, from.column + 2);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row + 1, from.column - 2);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 1, from.column + 2);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 1, from.column - 2);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 2, from.column + 1);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
            {
                let to: Square = Square::new(from.row - 2, from.column - 1);
                if to.is_valid() && !self.black_pieces().get_square(to) {
                    vec.push(Move::new(from, to));
                }
            }
        }
        vec
    }

    pub fn generate_black_king_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_king.to_squares() {
            for r in -1i32..2 {
                for c in -1i32..2 {
                    if r == 0 && c == 0 { continue; }
                    let to: Square = Square::new(from.row + r, from.column + c);
                    if self.black_pieces().get_square(to) { continue; }
                    if !to.is_valid() { continue; }
                    vec.push(Move::new(from, to));
                }
            }
        }
        if  !self.moved_pieces.get_square(square_from_string("E8".to_string())) &&
            !self.moved_pieces.get_square(square_from_string("H8".to_string())) &&
            self.empty_squares().get_square(square_from_string("F8".to_string())) &&
            self.empty_squares().get_square(square_from_string("G8".to_string())) {
                vec.push(Move::new(square_from_string("E8".to_string()), square_from_string("G8".to_string())));
            }

        if  !self.moved_pieces.get_square(square_from_string("E8".to_string())) &&
            !self.moved_pieces.get_square(square_from_string("A8".to_string())) &&
            self.empty_squares().get_square(square_from_string("B8".to_string())) &&
            self.empty_squares().get_square(square_from_string("C8".to_string())) &&
            self.empty_squares().get_square(square_from_string("D8".to_string())) {
                vec.push(Move::new(square_from_string("E8".to_string()), square_from_string("C8".to_string())));
            }
        
        vec
    }

    pub fn generate_black_rook_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_rook.to_squares() {
            for r in 1..(8-from.row) {
                let to: Square = Square::new(from.row+r, from.column);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for r in 1..(from.row + 1) {
                let to: Square = Square::new(from.row-r, from.column);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for c in 1..(8-from.column) {
                let to: Square = Square::new(from.row, from.column + c);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for c in 1..(from.column + 1) {
                let to: Square = Square::new(from.row, from.column - c);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn generate_black_bishop_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_bishop.to_squares() {
            for i in 1..(8 - cmp::max(from.row, from.column)) {
                let to: Square = Square::new(from.row + i, from.column + i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row, from.column) + 1) {
                let to: Square = Square::new(from.row - i, from.column - i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row + 1, 8 - from.column)) {
                let to: Square = Square::new(from.row - i, from.column + i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(8 - from.row, from.column + 1)) {
                let to: Square = Square::new(from.row + i, from.column - i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn generate_black_queen_moves(&self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::new();
        for from in self.black_queen.to_squares() {
            for r in 1..(8-from.row) {
                let to: Square = Square::new(from.row+r, from.column);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for r in 1..(from.row + 1) {
                let to: Square = Square::new(from.row-r, from.column);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for c in 1..(8-from.column) {
                let to: Square = Square::new(from.row, from.column + c);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for c in 1..(from.column + 1) {
                let to: Square = Square::new(from.row, from.column - c);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(8 - cmp::max(from.row, from.column)) {
                let to: Square = Square::new(from.row + i, from.column + i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row, from.column) + 1) {
                let to: Square = Square::new(from.row - i, from.column - i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(from.row + 1, 8 - from.column)) {
                let to: Square = Square::new(from.row - i, from.column + i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
            for i in 1..(cmp::min(8 - from.row, from.column + 1)) {
                let to: Square = Square::new(from.row + i, from.column - i);
                if self.black_pieces().get_square(to) { break; }
                vec.push(Move::new(from, to));
                if self.white_pieces().get_square(to) { break; }
            }
        }
        vec
    }

    pub fn get_square_piece(&self, square: Square) -> Piece {
        if self.white_pawn.get_square(square)        { return Piece::Pawn; }
        if self.white_rook.get_square(square)   { return Piece::Rook; }
        if self.white_knight.get_square(square) { return Piece::Knight; }
        if self.white_bishop.get_square(square) { return Piece::Bishop; }
        if self.white_queen.get_square(square)  { return Piece::Queen; }
        if self.white_king.get_square(square)   { return Piece::King;}
        if self.black_pawn.get_square(square)   { return Piece::Pawn; }
        if self.black_rook.get_square(square)   { return Piece::Rook;}
        if self.black_knight.get_square(square) { return Piece::Knight; }
        if self.black_bishop.get_square(square) { return Piece::Bishop; }
        if self.black_queen.get_square(square)  { return Piece::Queen; }
        if self.black_king.get_square(square)   { return Piece::King;}
        Piece::None
    }

    pub fn get_square_color(&self, square: Square) -> Color {
        if self.white_pieces().get_square(square) { return Color::White; }
        if self.black_pieces().get_square(square) { return Color::Black; }
        Color::None
    }

    pub fn get_legal_moves_from_square(&self, from: Square) -> Vec<Move> {
        self.generate_moves().into_iter().filter(|m| m.from == from).collect::<Vec<Move>>()
    }

    pub fn make_move_string(&mut self, from: &str, to: &str) {
        self.make_move(Move::new(square_from_string(from.to_string()), square_from_string(to.to_string())), true);
    }

    pub fn make_move(&mut self, _move: Move, validate: bool) {
        if validate && !self.is_move_valid(&_move) { return; }
        let capture: bool = !self.empty_squares().get_square(_move.to);
        self.  white_pawn.remove_square(_move.to);
        self.  white_rook.remove_square(_move.to);
        self.white_knight.remove_square(_move.to);
        self.white_bishop.remove_square(_move.to);
        self. white_queen.remove_square(_move.to);
        self.  white_king.remove_square(_move.to);
        self.  black_pawn.remove_square(_move.to);
        self.  black_rook.remove_square(_move.to);
        self.black_knight.remove_square(_move.to);
        self.black_bishop.remove_square(_move.to);
        self. black_queen.remove_square(_move.to);
        self.  black_king.remove_square(_move.to);

        if self.white_pawn.get_square(_move.from) { 
            if _move.from.row == 4 {
                if _move.to.column == _move.from.column + 1 && !capture {
                    self.black_pawn.remove_square(square_from_i32(_move.from.to_i32() + 1))
                }
                if _move.to.column == _move.from.column - 1 && !capture {
                    self.black_pawn.remove_square(square_from_i32(_move.from.to_i32() - 1))
                }
            }

            if _move.to.row == 7 {
                self.white_queen.set_square(_move.to);
            } else {
                self.white_pawn.set_square(_move.to); 
            }
            self.moved_pieces.set_square(_move.from);
        }
        else if self.white_rook.get_square(_move.from) {
            self.white_rook.set_square(_move.to);
            self.moved_pieces.set_square(_move.from);
        }
        else if self.white_knight.get_square(_move.from) {self.white_knight.set_square(_move.to); }
        else if self.white_bishop.get_square(_move.from) {self.white_bishop.set_square(_move.to); }
        else if self.white_queen.get_square(_move.from) {self.white_queen.set_square(_move.to); }
        else if self.white_king.get_square(_move.from) {
            self.white_king.set_square(_move.to); 
            if _move.from == square_from_string("E1".to_string()) {
                if _move.to == square_from_string("C1".to_string()) {
                    self.white_rook.remove_square(square_from_string("A1".to_string()));
                    self.white_rook.set_square(square_from_string("D1".to_string()));
                }
                if _move.to == square_from_string("G1".to_string()) {
                    self.white_rook.remove_square(square_from_string("H1".to_string()));
                    self.white_rook.set_square(square_from_string("F1".to_string()));
                }
            }
            self.moved_pieces.set_square(_move.from);
        }
        else if self.black_pawn.get_square(_move.from) { 
            if _move.from.row == 3 {
                if _move.to.column == _move.from.column + 1 && !capture {
                    self.white_pawn.remove_square(square_from_i32(_move.from.to_i32() + 1))
                }
                if _move.to.column == _move.from.column - 1 && !capture {
                    self.white_pawn.remove_square(square_from_i32(_move.from.to_i32() - 1))
                }
            }

            if _move.to.row == 0 {
                self.black_queen.set_square(_move.to);
            } else {
                self.black_pawn.set_square(_move.to); 
            }
            self.moved_pieces.set_square(_move.from);
        }
        else if self.black_rook.get_square(_move.from) {
            self.black_rook.set_square(_move.to);
            self.moved_pieces.set_square(_move.from); 
        }
        else if self.black_knight.get_square(_move.from) {self.black_knight.set_square(_move.to); }
        else if self.black_bishop.get_square(_move.from) {self.black_bishop.set_square(_move.to); }
        else if self.black_queen.get_square(_move.from) {self.black_queen.set_square(_move.to); }
        else if self.black_king.get_square(_move.from) {
            self.black_king.set_square(_move.to); 
            if _move.from == square_from_string("E8".to_string()) {
                if _move.to == square_from_string("C8".to_string()) {
                    self.black_rook.remove_square(square_from_string("A8".to_string()));
                    self.black_rook.set_square(square_from_string("D8".to_string()));
                }
                if _move.to == square_from_string("G8".to_string()) {
                    self.black_rook.remove_square(square_from_string("H8".to_string()));
                    self.black_rook.set_square(square_from_string("F8".to_string()));
                }
            }
            self.moved_pieces.set_square(_move.from);
        }

        self.  white_pawn.remove_square(_move.from);
        self.  white_rook.remove_square(_move.from);
        self.white_knight.remove_square(_move.from);
        self.white_bishop.remove_square(_move.from);
        self. white_queen.remove_square(_move.from);
        self.  white_king.remove_square(_move.from);
        self.  black_pawn.remove_square(_move.from);
        self.  black_rook.remove_square(_move.from);
        self.black_knight.remove_square(_move.from);
        self.black_bishop.remove_square(_move.from);
        self. black_queen.remove_square(_move.from);
        self.  black_king.remove_square(_move.from);
        
        self.white_turn = !self.white_turn;
    }

    pub fn is_white_checked(&self) -> bool {
        let mut bb: BitBoard = BitBoard::new(0);
        for m in self.generate_black_rook_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_black_knight_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_black_bishop_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_black_queen_moves() {
            bb.set_square(m.to);
        }
        for s in self.generate_black_pawn_threats() {
            bb.set_square(s);
        }

        (bb.value & self.white_king.value) != 0
    }

    pub fn is_black_checked(&self) -> bool {
        let mut bb: BitBoard = BitBoard::new(0);
        for m in self.generate_white_rook_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_white_knight_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_white_bishop_moves() {
            bb.set_square(m.to);
        }
        for m in self.generate_white_queen_moves() {
            bb.set_square(m.to);
        }
        for s in self.generate_white_pawn_threats() {
            bb.set_square(s);
        }

        (bb.value & self.black_king.value) != 0
    }

    pub fn is_white_mated(&self) -> bool {
        self.white_turn && self.is_white_checked() && self.generate_moves().len() == 0
    }

    pub fn is_black_mated(&self) -> bool {
        !self.white_turn && self.is_black_checked() && self.generate_moves().len() == 0
    }

    pub fn is_white_stalemate(&self) -> bool {
        self.white_turn && !self.is_white_checked() && self.generate_moves().len() == 0
    }

    pub fn is_black_stalemate(&self) -> bool {
        !self.white_turn && !self.is_black_checked() && self.generate_moves().len() == 0
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn move_king() {
        let mut b = ChessBoard::new();
        b.make_move_string("E2", "E4");
        b.make_move_string("E7", "E5");
        for m in b.generate_moves() {
            println!("{} -> {}", m.from.to_string(), m.to.to_string());
        }
        b.print();
        
    }
}
