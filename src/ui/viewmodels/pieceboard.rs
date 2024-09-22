
use crate::constants::Piece;
use crate::constants::Color;

#[derive(Debug, Default)]
pub struct PieceBoard {
    board: [[Occupant; 8]; 8],
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Occupant {
    Occupied(Piece, Color),
    #[default] Empty
}

impl Occupant {
    pub const fn empty() -> Self {
        Self::Empty
    }

    pub const fn white(piece: Piece) -> Self {
        Self::Occupied(piece, Color::WHITE)
    }

    pub const fn black(piece: Piece) -> Self {
        Self::Occupied(piece, Color::BLACK)
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Occupant::Occupied(_, color) => Some(*color),
            Occupant::Empty => None
        }
    }

    pub fn piece(&self) -> Option<Piece> {
        match self {
            Occupant::Occupied(piece, _) => Some(*piece),
            Occupant::Empty => None
        }
    }
}

pub const START_POSITION : PieceBoard = PieceBoard {
    board: [
        [ Occupant::black(Piece::Rook) , Occupant::black(Piece::Knight) , Occupant::black(Piece::Bishop) , Occupant::black(Piece::Queen) , Occupant::black(Piece::King) , Occupant::black(Piece::Bishop) , Occupant::black(Piece::Knight) , Occupant::black(Piece::Rook) ] ,
        [ Occupant::black(Piece::Pawn) , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)  , Occupant::black(Piece::Pawn) , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn) ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::white(Piece::Pawn) , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)  , Occupant::white(Piece::Pawn) , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn) ] ,
        [ Occupant::white(Piece::Rook) , Occupant::white(Piece::Knight) , Occupant::white(Piece::Bishop) , Occupant::white(Piece::Queen) , Occupant::white(Piece::King) , Occupant::white(Piece::Bishop) , Occupant::white(Piece::Knight) , Occupant::white(Piece::Rook) ] ,
    ]
};

impl PieceBoard {
    pub fn new() -> Self {
        Self {
            board: [[Occupant::empty(); 8]; 8]
        }
    }

    pub fn set_board(&mut self, board: [[Occupant; 8]; 8]) {
        self.board = board;
    }

    pub fn set_startpos(&mut self) {
        self.set_board(START_POSITION.board);
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for row in self.board.iter() {
            let mut empty = 0;
            for occupant in row.iter() {
                match occupant {
                    Occupant::Occupied(piece, color) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(piece.to_fen(*color));
                    },
                    Occupant::Empty => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            fen.push('/');
        }
        fen.pop(); // remove the last '/'
        fen
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = [[Occupant::empty(); 8]; 8];
        let mut row = 0;
        let mut col = 0;
        for c in fen.chars() {
            match c {
                'r' => { board[row][col] = Occupant::black(Piece::Rook); col += 1; },
                'n' => { board[row][col] = Occupant::black(Piece::Knight); col += 1; },
                'b' => { board[row][col] = Occupant::black(Piece::Bishop); col += 1; },
                'q' => { board[row][col] = Occupant::black(Piece::Queen); col += 1; },
                'k' => { board[row][col] = Occupant::black(Piece::King); col += 1; },
                'p' => { board[row][col] = Occupant::black(Piece::Pawn); col += 1; },
                'R' => { board[row][col] = Occupant::white(Piece::Rook); col += 1; },
                'N' => { board[row][col] = Occupant::white(Piece::Knight); col += 1; },
                'B' => { board[row][col] = Occupant::white(Piece::Bishop); col += 1; },
                'Q' => { board[row][col] = Occupant::white(Piece::Queen); col += 1; },
                'K' => { board[row][col] = Occupant::white(Piece::King); col += 1; },
                'P' => { board[row][col] = Occupant::white(Piece::Pawn); col += 1; },
                '1' => { col += 1; },
                '2' => { col += 2; },
                '3' => { col += 3; },
                '4' => { col += 4; },
                '5' => { col += 5; },
                '6' => { col += 6; },
                '7' => { col += 7; },
                '8' => { col += 8; },
                '/' => { row += 1; col = 0; },
                _ => { panic!("Invalid FEN character: {}", c); }
            }
        }
        Self { board }
    }

    pub fn alter(&mut self, moves: Vec<Alteration>) {
        for m in moves {
            match m {
                Alteration::Move(from, to) => {
                    self.board[to.0][to.1] = self.board[from.0][from.1];
                    self.board[from.0][from.1] = Occupant::empty();
                },
                Alteration::Remove(pos) => {
                    self.board[pos.0][pos.1] = Occupant::empty();
                },
                Alteration::Place(pos, occupant) => {
                    self.board[pos.0][pos.1] = occupant;
                }
            }
        }
    }
}

pub enum Alteration {
    Move((usize, usize), (usize, usize)),
    Remove((usize, usize)),
    Place((usize, usize), Occupant)
}

#[cfg(test)]
mod tests {

    use super::*;

    mod fen {
        use super::*;

        #[test]
        pub fn converts_start_position_correctly() {
            let mut board = PieceBoard::new();
            board.set_startpos();
            let fen = board.to_fen();
            assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        }

        #[test]
        pub fn converts_empty_board_correctly() {
            let board = PieceBoard::new();
            let fen = board.to_fen();
            assert_eq!(fen, "8/8/8/8/8/8/8/8");
        }

        #[test]
        pub fn converts_fen_to_board_correctly() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
            let board = PieceBoard::from_fen(fen);
            let fen2 = board.to_fen();
            assert_eq!(fen, fen2);
        }

        /* For want of a FEN type and an Arbitrary instance 
        #[quickcheck]
        pub fn converts_fen_to_board_correctly_quickcheck(fen: FEN) -> bool {
            let board = PieceBoard::from_fen(&fen);
            let fen2 = board.to_fen();
            fen == fen2
        }
        */
    }

    mod alter {
        use super::*;

        #[test]
        pub fn alters_board_correctly() {
            let mut board = PieceBoard::new();
            board.set_startpos();
            let moves = vec![
                Alteration::Move((1, 0), (2, 0)),
                Alteration::Remove((7, 4)),
                Alteration::Place((4, 4), Occupant::black(Piece::Pawn))
            ];
            board.alter(moves);
            let fen = board.to_fen();
            assert_eq!(fen, "rnbqkbnr/1ppppppp/p7/8/4p3/8/PPPPPPPP/RNBQ1BNR");
        }
    }
}
