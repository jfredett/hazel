
use super::*;

use bitboard::Bitboard;
use constants::*;

mod debug;

bitflags! {
    pub struct Metadata: u8 {
        const WHITE_CASTLE_LONG  = 0b00000001;
        const WHITE_CASTLE_SHORT = 0b00000010;
        const BLACK_CASTLE_LONG  = 0b00000100;
        const BLACK_CASTLE_SHORT = 0b00001000;
        const EN_PASSANT         = 0b00010000;
        const BLACK_TO_MOVE      = 0b00100000;
        const IN_CHECK           = 0b01000000;
        const UNUSED             = 0b01000000;
        // convenience flags
        const DEFAULT            = 0b00001111;
        const BLANK              = 0b00000000;
    }
}
#[derive(PartialEq, Eq, Hash)]
pub struct Ply {
    // indexed by COLOR
    pawns: [Bitboard; 2],
    kings: [Bitboard; 2],
    queens: [Bitboard; 2],
    // indexed by COLOR, then it's a/h rook
    rooks: [Bitboard; 2],
    bishops: [Bitboard; 2],
    knights: [Bitboard; 2],
    en_passant: Option<Bitboard>,
    full_move_clock: u32, // we're aligned to 64b, so this is the biggest that'll fit conveniently
    half_move_clock: u8, // this is for the 50m rule
    meta: Metadata,
}

// parse a fen string and construct the ply
impl Ply {
    pub fn empty() -> Ply {
        Ply {
            pawns: [Bitboard::empty(); 2],
            kings: [Bitboard::empty(); 2],
            queens: [Bitboard::empty(); 2],
            rooks: [Bitboard::empty(); 2],
            bishops: [Bitboard::empty(); 2],
            knights: [Bitboard::empty(); 2],
            en_passant: None,
            meta: Metadata::DEFAULT,
            half_move_clock: 0,
            full_move_clock: 1
        }
    }

    pub fn from_fen(fen: String) -> Ply {
        // A cheap and cheerful fen parser, very little error handling
        let fen_parts : Vec<&str> = fen.split(' ').collect();
        let mut ply = Ply::empty();

        // Board setup
        let mut rank = 7;
        let mut file = 0;
        for ch in fen_parts[0].chars() {
            match ch {
                'p' => { ply.pawns[Color::BLACK as usize].set(rank,file);   file += 1; }
                'k' => { ply.kings[Color::BLACK as usize].set(rank,file);   file += 1; }
                'q' => { ply.queens[Color::BLACK as usize].set(rank,file);  file += 1; }
                'r' => { ply.rooks[Color::BLACK as usize].set(rank,file);   file += 1; }
                'b' => { ply.bishops[Color::BLACK as usize].set(rank,file); file += 1; }
                'n' => { ply.knights[Color::BLACK as usize].set(rank,file); file += 1; }
                'P' => { ply.pawns[Color::WHITE as usize].set(rank,file);   file += 1; }
                'K' => { ply.kings[Color::WHITE as usize].set(rank,file);   file += 1; }
                'Q' => { ply.queens[Color::WHITE as usize].set(rank,file);  file += 1; }
                'R' => { ply.rooks[Color::WHITE as usize].set(rank,file);   file += 1; }
                'B' => { ply.bishops[Color::WHITE as usize].set(rank,file); file += 1; }
                'N' => { ply.knights[Color::WHITE as usize].set(rank,file); file += 1; }
                '/' => { rank -= 1; file = 0; }
                '1' => { file += 1; }
                '2' => { file += 2; }
                '3' => { file += 3; }
                '4' => { file += 4; }
                '5' => { file += 5; }
                '6' => { file += 6; }
                '7' => { file += 7; }
                '8' => { /* the next character will either be `/` or result in a different error */ }
                _ => { panic!("Invalid FEN board: {}", fen)}
            }

        }

        match fen_parts[1] {
            "w" => { /* intentionally blank */ }
            "b" => { ply.meta |= Metadata::BLACK_TO_MOVE; }
            _ => { panic!("Invalid FEN color: {}", fen); }
        };

        // castling rights
        for ch in fen_parts[2].chars() {
            match ch {
                'K' => { ply.meta |= Metadata::WHITE_CASTLE_SHORT; }
                'Q' => { ply.meta |= Metadata::WHITE_CASTLE_LONG; }
                'k' => { ply.meta |= Metadata::BLACK_CASTLE_SHORT; }
                'q' => { ply.meta |= Metadata::BLACK_CASTLE_LONG; }
                _ => { panic!("Invalid FEN castling key: {}", fen); }
            }
        }

        ply.en_passant = match fen_parts[3] {
            "-" => None,
            _ => Some(Bitboard::from_notation(fen_parts[3]))
        };

        ply.half_move_clock = fen_parts[4].parse().expect(&format!("Invalid FEN half-move: {}", fen));
        ply.full_move_clock = fen_parts[5].parse().expect(&format!("Invalid FEN full-move: {}", fen));



        ply
    }

    pub fn piece_at(&self, file: File, rank: usize, piece: Piece, color: Color) -> bool {
        if rank < 1 || rank > 8 { panic!("Invalid position {:?}{:?}", file, rank); }
        let board = match piece {
            Piece::Rook => { self.rooks[color as usize] }
            Piece::Bishop => { self.bishops[color as usize] }
            Piece::Knight => { self.knights[color as usize] }
            Piece::King => { self.kings[color as usize] }
            Piece::Queen => { self.queens[color as usize] }
            Piece::Pawn => { self.pawns[color as usize] }
        };
        board.is_set(rank - 1, file as usize)
    }
}



#[cfg(test)]
pub(crate) mod test {
    use super::*;

    pub fn start_position() -> Ply {
        Ply {
            pawns: [
                Bitboard::from(0x00_00_00_00_00_00_FF_00),
                Bitboard::from(0x00_FF_00_00_00_00_00_00)
            ],
            kings: [
                Bitboard::from_notation("e1"),
                Bitboard::from_notation("e8")
            ],
            queens: [
                Bitboard::from_notation("d1"),
                Bitboard::from_notation("d8")
            ],
            rooks: [
                Bitboard::from_notation("a1") | Bitboard::from_notation("h1"),
                Bitboard::from_notation("a8") | Bitboard::from_notation("h8")
            ],
            bishops: [
                Bitboard::from_notation("c1") | Bitboard::from_notation("f1"),
                Bitboard::from_notation("c8") | Bitboard::from_notation("f8")
            ],
            knights: [
                Bitboard::from_notation("b1")| Bitboard::from_notation("g1"),
                Bitboard::from_notation("b8")| Bitboard::from_notation("g8")
            ],
            en_passant: None,
            meta: Metadata::BLACK_CASTLE_LONG | Metadata::BLACK_CASTLE_SHORT |
                  Metadata::WHITE_CASTLE_LONG | Metadata::WHITE_CASTLE_SHORT,
            full_move_clock: 1,
            half_move_clock: 0
        }
    }

    mod piece_at {
        use super::*;    

        mod rooks {
            use super::*;

            #[test]
            fn sees_rooks_in_start_position() {                
                let ply = start_position();
                assert!(ply.piece_at(File::A, 1, Piece::Rook, Color::WHITE));
                assert!(ply.piece_at(File::H, 1, Piece::Rook, Color::WHITE));
                assert!(ply.piece_at(File::A, 8, Piece::Rook, Color::BLACK));
                assert!(ply.piece_at(File::H, 8, Piece::Rook, Color::BLACK));
            }            

            #[test]
            fn does_not_see_rook_where_there_are_no_rooks() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 2, Piece::Rook, Color::WHITE));
                assert!(!ply.piece_at(File::H, 7, Piece::Rook, Color::BLACK));
            }
        }

        mod bishops {
            use super::*;

            #[test]
            fn sees_bishops_in_start_position() {                
                let ply = start_position();
                assert!(ply.piece_at(File::C, 1, Piece::Bishop, Color::WHITE));
                assert!(ply.piece_at(File::F, 1, Piece::Bishop, Color::WHITE));
                assert!(ply.piece_at(File::C, 8, Piece::Bishop, Color::BLACK));
                assert!(ply.piece_at(File::F, 8, Piece::Bishop, Color::BLACK));
            }            

            #[test]
            fn does_not_see_bishop_where_there_are_no_bishops() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 2, Piece::Bishop, Color::WHITE));
                assert!(!ply.piece_at(File::H, 4, Piece::Bishop, Color::BLACK));
            }
        }

        mod knights {
            use super::*;

            #[test]
            fn sees_knights_in_start_position() {                
                let ply = start_position();
                assert!(ply.piece_at(File::B, 1, Piece::Knight, Color::WHITE));
                assert!(ply.piece_at(File::G, 1, Piece::Knight, Color::WHITE));
                assert!(ply.piece_at(File::B, 8, Piece::Knight, Color::BLACK));
                assert!(ply.piece_at(File::G, 8, Piece::Knight, Color::BLACK));
            }            

            #[test]
            fn does_not_see_knight_where_there_are_no_knights() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 3, Piece::Knight, Color::WHITE));
                assert!(!ply.piece_at(File::H, 6, Piece::Knight, Color::BLACK));
            }
        }

        mod king {
            use super::*;

            #[test]
            fn sees_kings_in_start_position() {                
                let ply = start_position();
                assert!(ply.piece_at(File::E, 1, Piece::King, Color::WHITE));
                assert!(ply.piece_at(File::E, 8, Piece::King, Color::BLACK));
            }            

            #[test]
            fn does_not_see_bishop_where_there_are_no_bishops() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 2, Piece::King, Color::WHITE));
                assert!(!ply.piece_at(File::H, 2, Piece::King, Color::BLACK));
            }
        }

        mod queen {
            use super::*;

            #[test]
            fn sees_queens_in_start_position() {                
                let ply = start_position();
                assert!(ply.piece_at(File::D, 1, Piece::Queen, Color::WHITE));
                assert!(ply.piece_at(File::D, 8, Piece::Queen, Color::BLACK));
            }            

            #[test]
            fn does_not_see_bishop_where_there_are_no_bishops() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 2, Piece::Queen, Color::WHITE));
                assert!(!ply.piece_at(File::H, 2, Piece::King, Color::BLACK));
            }

        }

        mod pawns {
            use super::*;

            #[test]
            fn sees_the_white_pawns() {                
                let ply = start_position();
                assert!(ply.piece_at(File::A, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::B, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::C, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::D, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::E, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::F, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::G, 2, Piece::Pawn, Color::WHITE));
                assert!(ply.piece_at(File::H, 2, Piece::Pawn, Color::WHITE));
            }            

            #[test]
            fn sees_the_black_pawns() {                
                let ply = start_position();
                assert!(ply.piece_at(File::A, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::B, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::C, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::D, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::E, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::F, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::G, 7, Piece::Pawn, Color::BLACK));
                assert!(ply.piece_at(File::H, 7, Piece::Pawn, Color::BLACK));
            }            

            #[test]
            fn does_not_see_pawns_where_there_are_no_pawns() {
                let ply = start_position();
                assert!(!ply.piece_at(File::A, 5, Piece::Pawn, Color::WHITE));
                assert!(!ply.piece_at(File::A, 5, Piece::Pawn, Color::BLACK));
            }
        }
    }

    mod from_fen {
        use super::*;

        #[test]
        fn parses_starting_position_correctly() {
            let start_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let ply = Ply::from_fen(start_fen);
            assert_eq!(ply, start_position());
        }
    }
}