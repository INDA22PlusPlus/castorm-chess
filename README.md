# castorm-chess
 
 # Structs
 
 - Square\
 Represents a square on the chess board. Has two i32 values, row and column. The squares on the chess board are encoded from bottom-left to upper-right. So the bottom row is row 0 and the left-most column is column 0.
 
 - Move\
 Represents a chess move. Has two Square values, from and to. 'from' is the Square that a piece was moved from and 'to' is the Square that it moved to. Currently there are no special moves implemented so this is suficcient to describe all possible moves that are implemented so far.
 
 - BitBoard\
 Represents encoded data, mainly for the position of pieces. Has one u64 value, value. Every bit in the value corresponds to a Square on the chess board. Bit 0, or the LSB, corresponds the Square with row = 0 and column = 0. Bit 1 corresponds to the Square with row = 0 and column = 1. Generally, bit n corresponds to the Square with values row = n / 8 and column = n % 8
 
 - ChessBoard\
 Represents the chess board. Consists of a bool describing whether or not it is white's turn to play and 12 BitBoards, each describing the current position of each piece type, the piece types being white pawn, white rook, white knight, white bishop, white queen, white king and the same pieces for black. Together these BitBoards describe the whole chess board. 
 
 
 # Functions
 
 - square_from_i32(value: i32) -> Square\
 Creates the square that corresponds to a certain value (0 to 63). Encoding is the same as descibed above under BitBoard.
 
 - square_from_string(value: String) -> Square\
 Creates the square that is represented by standard chess notation, for example A1, B1 or H8. Square A1 is the same as Square 0, Square B1 is the same as Square 1 and Square H8 is the same as Square 63.
 
 
 # ChessBoard impl
 
 - new() -> ChessBoard\
 A chess board can be instantiated through ChessBoard::new(); which initializes the chess board to its starting position according to standard chess rules.
 
 - generate_moves(&self) -> Vec<Move>\
 Generates all legal moves in the current position and returns them in a vector.
 
 - make_move(&self, m: Move)\
 Applies the move to the chess board (currently even if it is an illegal move).
 
 - is_white_checked(&self) -> bool, is_black_checked(&self) -> bool, is_white_mated(&self) -> bool, is_black_mated(&self) -> bool\
 Self explanatory.
 
 - white_pieces(&self) -> BitBoard\
 Generates a BitBoard representing all white pieces
 
 - black_pieces(&self) -> BitBoard\
 Generates a BitBoard representing all black pieces
 
 - empty_squares(&self) -> BitBoard\
 Generates a BitBoard representing all empty squares
 
 - is_move_valid(&self, m: &Move) -> bool\
 Whether or not a given Move is valid
 
 - print(&self)\
 For debugging purposes, prints the chess board to stdout
 
 All possible moves for each piece type can also be generated with logical method names (for example generate_white_pawn_moves(&self) -> Vec<Move>)\
 
 Every BitBoard in a ChessBoard is public and can thus be accessed directly and modified through the BitBoard impl:\
 
 # BitBoard impl
 
 - get_square(&self, square: Square) -> bool\
 returns true if the bit corresponding to the Square square is set to 1, otherwise false
 
 - set_square(&self, square: Square)\
 sets the bit corresponding to the Square square to 1
 
 - remove_square(&self, square: Square)\
 sets the bit corresponding to the Square square to 0
 
 - to_squares(&self) -> Vec<Squares>\
 generates a vector of every Squares who's corresponding bit is set to 1 in the BitBoard
