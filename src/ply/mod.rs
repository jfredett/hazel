#![allow(non_snake_case)]
use crate::{movement::Move, moveset::MoveSet};

use super::*;

use bitboard::Bitboard;
use constants::*;
use serde::{Deserialize, Serialize, de::IntoDeserializer};

mod debug;
mod creation;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Metadata: u8 {
        const WHITE_CASTLE_LONG  = 0b00000001;
        const WHITE_CASTLE_SHORT = 0b00000010;
        const BLACK_CASTLE_LONG  = 0b00000100;
        const BLACK_CASTLE_SHORT = 0b00001000;
        const EN_PASSANT         = 0b00010000;
        const BLACK_TO_MOVE      = 0b00100000;
        const IN_CHECK           = 0b01000000;
        // NOTE: Maybe this should be used for 'seen this position before?' if 1, then we need to lookup in the 3-fold transpo table or w/e
        const UNUSED             = 0b10000000;
        // convenience flags
        const DEFAULT            = 0b00001111;
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Ply {
    // indexed by COLOR
    pub pawns: [Bitboard; 2],
    pub kings: [Bitboard; 2],
    pub queens: [Bitboard; 2],
    // indexed by COLOR, then it's a/h rook
    pub rooks: [Bitboard; 2],
    pub bishops: [Bitboard; 2],
    pub knights: [Bitboard; 2],
    pub en_passant: Option<Bitboard>,
    pub full_move_clock: u32, // we're aligned to 64b, so this is the biggest that'll fit conveniently
    // NOTE: Maybe mask this off to 6 bits (halfmove count should never go > 50), then use the top two bits for 3-fold repetition? Stick the whole thing
    // in the metadata struct?
    pub half_move_clock: u8, // this is for the 50m rule
    pub meta: Metadata,
}

// parse a fen string and construct the ply
impl Ply {
    /// Provides a bitboard which shows the location of all squares occupied by pieces of the given
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::bitboard::Bitboard;
    /// # use hazel::ply::Ply;
    /// # use hazel::constants::*;
    /// let fen = "8/5k1p/2n5/3N4/6P1/3K4/8/8 w - - 0 1".to_string();
    /// let ply = Ply::from_fen(&fen);
    /// let expected_occupancy_white = bitboard!("d3", "d5", "g4");
    /// let expected_occupancy_black = bitboard!("c6", "f7", "h7");
    /// assert_eq!(ply.occupancy_for(Color::WHITE), expected_occupancy_white);
    /// assert_eq!(ply.occupancy_for(Color::BLACK), expected_occupancy_black);
    /// ```
    pub fn occupancy_for(&self, color: Color) -> Bitboard {
        self.kings[color as usize]   |
        self.queens[color as usize]  |
        self.rooks[color as usize]   |
        self.bishops[color as usize] |
        self.knights[color as usize] |
        self.pawns[color as usize]
    }
    
    /// 
    pub fn moves(&self) -> MoveSet {
        Move::generate(self, self.current_player())
    }
    
    /// True if the current player both has the right to castle long and the ability.
    pub fn can_castle_long(&self) -> bool {
        match self.current_player() {
            Color::WHITE => { 
                self.meta.contains(Metadata::WHITE_CASTLE_LONG) & (self.occupancy() & bitboard!("b1", "c1", "d1")).is_empty()
            }
            Color::BLACK => { 
                self.meta.contains(Metadata::BLACK_CASTLE_LONG) & (self.occupancy() & bitboard!("b8", "c8", "d8")).is_empty()
            }
        }
    }
    
    /// True if the current player both has the right to castle short and the ability.
    pub fn can_castle_short(&self) -> bool {
        match self.current_player() {
            Color::WHITE => { 
                self.meta.contains(Metadata::WHITE_CASTLE_SHORT) & (self.occupancy() & bitboard!("f1", "g1")).is_empty()
            }
            Color::BLACK => { 
                self.meta.contains(Metadata::BLACK_CASTLE_SHORT) & (self.occupancy() & bitboard!("f8", "g8")).is_empty()
            }
        }
    }
    
    /// Provides a bitboard which shows the location of all squares occupied by pieces of any
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::bitboard::Bitboard;
    /// # use hazel::ply::Ply;
    /// # use hazel::constants::*;
    /// let fen = "8/5k1p/2n5/3N4/6P1/3K4/8/8 w - - 0 1".to_string();
    /// let ply = Ply::from_fen(&fen);
    /// let expected_occupancy_white = bitboard!("d3", "d5", "g4");
    /// let expected_occupancy_black = bitboard!("c6", "f7", "h7");
    /// assert_eq!(ply.occupancy(), expected_occupancy_white | expected_occupancy_black);
    /// ```
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy_for(Color::WHITE) | self.occupancy_for(Color::BLACK)
    }
    
    /// Returns the color of the player who will make the next move.
    pub fn current_player(&self) -> Color {
        if self.meta.contains(Metadata::BLACK_TO_MOVE) {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
    
    /// Returns the color of the player who is not currently making the next move.
    pub fn inactive_player(&self) -> Color {
        if self.meta.contains(Metadata::BLACK_TO_MOVE) {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    pub fn piece_at(&self, file: File, rank: usize, piece: Piece, color: Color) -> bool {
        if !(1..=8).contains(&rank) { panic!("Invalid position {:?}{:?}", file, rank); }
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
    
    /// Returns the piece at the index provided, if no piece is present, returns none.
    pub fn piece_at_index(&self, idx: usize) -> Option<(Color, Piece)> {
        for color in COLORS {
            if self.rooks[color as usize].is_index_set(idx) { return Some((color, Piece::Rook)) }
            if self.bishops[color as usize].is_index_set(idx) { return Some((color, Piece::Bishop)) }
            if self.knights[color as usize].is_index_set(idx) { return Some((color, Piece::Knight)) }
            if self.kings[color as usize].is_index_set(idx) { return Some((color, Piece::King)) }
            if self.queens[color as usize].is_index_set(idx) { return Some((color, Piece::Queen)) }
            if self.pawns[color as usize].is_index_set(idx) { return Some((color, Piece::Pawn)) }
        }
        None
    }
    
    pub fn make(&mut self, mov: Move) {
        if mov.is_short_castle() {
            match self.current_player() {
                Color::WHITE => {
                    self.kings[Color::WHITE as usize].unset_by_index(0o04);
                    self.kings[Color::WHITE as usize].set_by_index(0o06);
                    self.rooks[Color::WHITE as usize].unset_by_index(0o07);
                    self.rooks[Color::WHITE as usize].set_by_index(0o05);
                },
                Color::BLACK => {
                    self.kings[Color::BLACK as usize].unset_by_index(0o74);
                    self.kings[Color::BLACK as usize].set_by_index(0o76);
                    self.rooks[Color::BLACK as usize].unset_by_index(0o77);
                    self.rooks[Color::BLACK as usize].set_by_index(0o75);
                }
            }
        } else if mov.is_long_castle() {
            match self.current_player() {
                Color::WHITE => {
                    self.kings[Color::WHITE as usize].unset_by_index(0o04);
                    self.kings[Color::WHITE as usize].set_by_index(0o02);
                    self.rooks[Color::WHITE as usize].unset_by_index(0o00);
                    self.rooks[Color::WHITE as usize].set_by_index(0o03);
                },
                Color::BLACK => {
                    self.kings[Color::BLACK as usize].unset_by_index(0o74);
                    self.kings[Color::BLACK as usize].set_by_index(0o72);
                    self.rooks[Color::BLACK as usize].unset_by_index(0o70);
                    self.rooks[Color::BLACK as usize].set_by_index(0o73);
                }
            }
        } else if let Some((color, piece_in)) = self.piece_at_index(mov.source_idx().into()) {
            let piece = if mov.is_promotion() {
                mov.promotion_piece()
            } else {
                piece_in
            };

            if mov.is_capture() {
                // remove the piece of the other color at target_idx
                if let Some((other_color, target_piece)) = self.piece_at_index(mov.target_idx().into()) {
                    self.get_mut_piece(other_color, target_piece).unset_by_index(mov.target_idx().into());
                } else {
                    dbg!(mov);
                    dbg!(&self);
                    dbg!(&self.to_fen());
                    unimplemented!();
                }
            } 

            self.get_mut_piece(color, piece).unset_by_index(mov.source_idx().into());
            self.get_mut_piece(color, piece).set_by_index(mov.target_idx().into());
        } else {
            // TODO: Promotions
            dbg!(mov);
            panic!("Could not find piece at index: {}", mov.source_idx());
        }
        
        // just completed black's turn, another full move down
        if self.meta.contains(Metadata::BLACK_TO_MOVE) { self.full_move_clock += 1; }

        /* TODO: Half-move clock
        // Halfmove count goes up every move and resets on pawn move or capture
        self.half_move_clock += 1;
        // FIXME: This is gross.
        if self.piece_at_index(mov.source_idx().into()).unwrap().1 == Piece::Pawn || mov.is_capture() { self.half_move_clock = 0; }
        */

        // flip the player-turn bit
        self.meta ^= Metadata::BLACK_TO_MOVE;
    }
    
    pub fn unmake(&mut self, mov: Move, captured_piece: Option<(Color, Piece)>) {

        let pat = self.piece_at_index(mov.target_idx().into());
        if pat.is_none() {
            dbg!(mov);
            dbg!(self);
            unimplemented!()
        }
        let (color, piece) = pat.unwrap();

        let rank_mask = if self.current_player() == Color::BLACK { 0o07 } else { 0o00 };

        if mov.is_short_castle() {
            self.kings[color as usize].move_piece(0o04 | rank_mask, 0o06 | rank_mask);
            self.rooks[color as usize].move_piece(0o07 | rank_mask, 0o05 | rank_mask);
        } else if mov.is_long_castle() {
            self.kings[color as usize].move_piece(0o04 | rank_mask, 0o02 | rank_mask);
            // NOTE: since the source for the rook is 0, we can omit the 0o00.
            self.rooks[color as usize].move_piece(rank_mask, 0o03 | rank_mask);
        } else if mov.is_promotion() {
            self.get_mut_piece(color, mov.promotion_piece()).unset_by_index(mov.target_idx().into());
        } else {
            if mov.is_capture() {
                // put the piece back
                let (other_color, cap_piece) = captured_piece.unwrap();
                self.get_mut_piece(other_color, cap_piece).set_by_index(mov.target_idx().into());
            }
            
            if mov.is_promotion() {
                // remove the promotion piece
                self.get_mut_piece(color, mov.promotion_piece()).unset_by_index(mov.target_idx().into());
            }

            // move the piece back to it's source square
            self.get_mut_piece(color, piece).move_piece(mov.target_idx().into(), mov.source_idx().into());
        }

        // if it was white's turn, then we need to decrement the clock
        if !self.meta.contains(Metadata::BLACK_TO_MOVE) { self.full_move_clock -= 1; }
        
        /* TODO:
        Half-move clock
        */
        

        // flip the player-turn bit
        self.meta ^= Metadata::BLACK_TO_MOVE;
    }
    
    /// A helper for digging into the ply structure to touch the right pieces.
    pub fn get_mut_piece(&mut self, color: Color, piece: Piece) -> &mut Bitboard {
        match piece {
            Piece::Knight => &mut self.knights[color as usize],
            Piece::Bishop => &mut self.bishops[color as usize],
            Piece::Rook   => &mut self.rooks[color as usize],
            Piece::Queen  => &mut self.queens[color as usize],
            Piece::King   => &mut self.kings[color as usize],
            Piece::Pawn   => &mut self.pawns[color as usize],
        }
    }

    /// A helper for digging into the ply structure to touch the right pieces.
    pub fn get_piece(&self, color: Color, piece: Piece) -> Bitboard {
        match piece {
            Piece::Knight => self.knights[color as usize],
            Piece::Bishop => self.bishops[color as usize],
            Piece::Rook   => self.rooks[color as usize],
            Piece::Queen  => self.queens[color as usize],
            Piece::King   => self.kings[color as usize],
            Piece::Pawn   => self.pawns[color as usize],
        }
    }


    /// returns an 8x8 array with characters representing each piece in the proper locations
    fn board_buffer(&self) -> [[char; 8]; 8] {
        let mut buf = [['.'; 8]; 8];

        // Encode the board into a 8x8 array of chars.
        for rank in 0..8 {
            for file in FILES {
                for piece in PIECES {
                    for color in COLORS {
                        if self.piece_at(file, rank + 1, piece, color) {
                            buf[rank][file as usize] = ASCII_PIECE_CHARS[color as usize][piece as usize];
                        }
                    }
                }
            }
        }

        buf
    }
}



#[cfg(test)]
pub(crate) mod test {
    use super::*;
    
    // NOTE: these are left as functions because they are used to test the `from_fen` and `to_fen`
    // functions elsewhere. Most tests should use the constants defined in constants/test.rs
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
    pub fn london_position() -> Ply {
        Ply {
            pawns: [
                Bitboard::from_notation("a2") | Bitboard::from_notation("b2") | Bitboard::from_notation("c3") | 
                Bitboard::from_notation("d4") | Bitboard::from_notation("e3") | Bitboard::from_notation("f2") | 
                Bitboard::from_notation("g2") | Bitboard::from_notation("h3")
                ,
                Bitboard::from_notation("a7") | Bitboard::from_notation("b7") | Bitboard::from_notation("c5") | 
                Bitboard::from_notation("d5") | Bitboard::from_notation("e6") | Bitboard::from_notation("f7") | 
                Bitboard::from_notation("g7") | Bitboard::from_notation("h7")
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
                Bitboard::from_notation("f1") | Bitboard::from_notation("f4"),
                Bitboard::from_notation("c8") | Bitboard::from_notation("e7")
            ],
            knights: [
                Bitboard::from_notation("d2")| Bitboard::from_notation("f3"),
                Bitboard::from_notation("c6")| Bitboard::from_notation("f6")
            ],
            en_passant: None,
            meta: Metadata::BLACK_CASTLE_LONG | Metadata::BLACK_CASTLE_SHORT |
                  Metadata::WHITE_CASTLE_LONG | Metadata::WHITE_CASTLE_SHORT |
                  Metadata::BLACK_TO_MOVE,
            full_move_clock: 7,
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
                let ply = london_position();
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
}