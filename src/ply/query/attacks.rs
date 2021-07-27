use super::*;

use crate::constants::move_tables::KNIGHT_MOVES;


impl Ply {
    // ATTACK QUERIES
    //
    // Global-question type functions, "What squares are attacked?", "What squares are pinned?" etc.
    // Stuff that cares about color, but not piece type.
    
    /// A bitboard containing every square attacked by any piece of the given color
    pub fn attacked_squares_for(&self, color: Color) -> Bitboard {
        self.influenced_squares_for(color) & !self.occupancy_for(color)
    }
    
    /// A bitboard showing which pieces of a given color are 'defended' -- that is, can be
    /// recaptured by some friendly piece if captured by an enemy piece. Note that this does not
    /// account for whether the recapture would be _good_, only if it'd be possible.
    pub fn defended_pieces_for(&self, color: Color) -> Bitboard {
        self.influenced_squares_for(color) & self.occupancy_for(color) 
    }
    
    pub fn influenced_squares_for(&self, color: Color) -> Bitboard {
        self.knight_attack_board_for(color) |
        self.queen_attack_board_for(color)  |
        self.rook_attack_board_for(color)   |
        self.bishop_attack_board_for(color) |
        self.pawn_attack_board_for(color)   |
        self.king_attack_board_for(color)  
    }

    // ATTACK AND MOVE BOARDS
    //
    // Piece-local stuff functions, "What squares can a piece attack?", "What squares do my pawns
    // defend?", etc.
    pub fn king_attack_board_for(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let king = self.get_piece(color, Piece::King);
        for d in DIRECTIONS {
            attacks |= king.shift(d);
        }
        attacks
    }

    /// Calculates all squares attacked by pawn of the given color, does not account for friendly squares.
    pub fn pawn_attack_board_for(&self, color: Color) -> Bitboard {
        let pawns = self.get_piece(color, Piece::Pawn);
        let pre_attacks = pawns.shift(color.pawn_direction());
        let east_attacks = pre_attacks.shift(Direction::E);
        let west_attacks = pre_attacks.shift(Direction::W);
        
        east_attacks | west_attacks
    }

    /// Calculates all squares attacked by all knights of the given color. 
    pub fn knight_attack_board_for(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let knights = self.knights_for(color);
        for source in knights.all_set_indices() {
            attacks |= KNIGHT_MOVES[source] & !self.occupancy_for(color);
        }
        attacks
    }
    
    /// Calculates all squares attacked by all queens of the given color. 
    pub fn queen_attack_board_for(&self, color: Color)  -> Bitboard { self.slider_attacks_for(Piece::Queen, color) }
    /// Calculates all squares attacked by all rooks of the given color. 
    pub fn rook_attack_board_for(&self, color: Color)   -> Bitboard { self.slider_attacks_for(Piece::Rook, color) }
    /// Calculates all squares attacked by all bishops of the given color. 
    pub fn bishop_attack_board_for(&self, color: Color) -> Bitboard { self.slider_attacks_for(Piece::Bishop, color) }

    fn slider_attacks_for(&self, piece: Piece, color: Color) -> Bitboard {
        let mut attacks = Bitboard::empty();
        let piece_board = self.get_piece(color, piece);
        for source in piece_board.all_set_indices() {
            attacks |= pextboard::attacks_for(piece, source, self.occupancy());
        }
        
        attacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod attacked_squares_for {
        use super::*;
        
        
        fn test_position() -> Ply {
            Ply::from_fen("r4rk1/pR1n1ppp/4pn2/2qp4/8/3QPNP1/P1PN1PP1/R5K1 w - - 1 15")
        }
        
        // The formatting here is deliberately weird, it's so it matches up with the chessboard
        // layout.

        fn expected_attacks() -> Bitboard {
            bitboard!(
                      "b8",            
                "a7",       "c7", "d7",                   "h7", 
                "a6", "b6",                         "g6",
                      "b5",       "d5", "e5", "f5", "g5",
                      "b4", "c4", "d4", "e4", "f4",       "h4",
                "a3", "b3", "c3",                         "h3",
                      "b2",             "e2",             "h2",
                      "b1", "c1", "d1", "e1", "f1",       "h1"
            )
        }
        
        fn expected_pawn_attacks() -> Bitboard {
            bitboard!(
                                  
                
               
              
                                  "d4",       "f4",       "h4",
                      "b3",       "d3", "e3", "f3", "g3", "h3"
                                                               
                                                              
            )
        }
        
        #[test]
        fn correctly_determines_all_attacked_squares() {
            assert_eq!(test_position().attacked_squares_for(Color::WHITE), expected_attacks());
        }

        #[test]
        fn correctly_determines_pawn_attacked_squares() {
            assert_eq!(test_position().pawn_attack_board_for(Color::WHITE), expected_pawn_attacks());
        }
        
    }
    
    
    mod defended_pieces_for {
        use super::*;

        #[test]
        fn the_queen_in_this_position_is_defended() {
            let p = Ply::from_fen("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
            let d = p.defended_pieces_for(Color::WHITE);
            assert!(p.defended_pieces_for(Color::WHITE).is_set(6,6));
        }
    }
    
}
