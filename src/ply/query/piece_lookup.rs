use super::*;

// TODO: There is a _ton_ of repetition in here that could be reduced.
//
impl Ply {
    /// True if the given piece of the given color is present at the given rank and file.
    // TODO: This is only used in tests and probably we should factor it out
    pub fn piece_at(&self, file: File, rank: usize, piece: Piece, color: Color) -> bool {
        if !(1..=8).contains(&rank) { panic!("Invalid position {:?}{:?}", file, rank); }
        let board = match piece {
            Piece::Rook => { self.rooks_for(color) }
            Piece::Bishop => { self.bishops_for(color) }
            Piece::Knight => { self.knights_for(color) }
            Piece::King => { self.king_for(color) }
            Piece::Queen => { self.queens_for(color) }
            Piece::Pawn => { self.pawns_for(color) }
        };
        board.is_set(rank - 1, file as usize)
    }
    
    /// Returns the piece at the index provided, if no piece is present, returns none.
    pub fn piece_at_index(&self, idx: usize) -> Option<(Color, Piece)> {
        for color in COLORS {
            if self.rooks_for(color).is_index_set(idx) { return Some((color, Piece::Rook)) }
            if self.bishops_for(color).is_index_set(idx) { return Some((color, Piece::Bishop)) }
            if self.knights_for(color).is_index_set(idx) { return Some((color, Piece::Knight)) }
            if self.king_for(color).is_index_set(idx) { return Some((color, Piece::King)) }
            if self.queens_for(color).is_index_set(idx) { return Some((color, Piece::Queen)) }
            if self.pawns_for(color).is_index_set(idx) { return Some((color, Piece::Pawn)) }
        }
        None
    }
    
    /// Returns the piece at the given index iff that piece is of the current player's color.
    pub fn friendly_piece_at_index(&self, idx: usize) -> Option<Piece> {
        if self.rooks_for( self.current_player() ).is_index_set(idx) { return Some(Piece::Rook) }
        if self.bishops_for( self.current_player() ).is_index_set(idx) { return Some(Piece::Bishop) }
        if self.knights_for( self.current_player() ).is_index_set(idx) { return Some(Piece::Knight) }
        if self.king_for( self.current_player() ).is_index_set(idx) { return Some(Piece::King) }
        if self.queens_for( self.current_player() ).is_index_set(idx) { return Some(Piece::Queen) }
        if self.pawns_for( self.current_player() ).is_index_set(idx) { return Some(Piece::Pawn) }
        None
    }
    
    /// Returns the piece at the given index iff that piece is of the other player's color.
    pub fn enemy_piece_at_index(&self, idx: usize) -> Option<Piece> {
        if self.rooks_for( self.other_player() ).is_index_set(idx) { return Some(Piece::Rook) }
        if self.bishops_for( self.other_player() ).is_index_set(idx) { return Some(Piece::Bishop) }
        if self.knights_for( self.other_player() ).is_index_set(idx) { return Some(Piece::Knight) }
        if self.king_for( self.other_player() ).is_index_set(idx) { return Some(Piece::King) }
        if self.queens_for( self.other_player() ).is_index_set(idx) { return Some(Piece::Queen) }
        if self.pawns_for( self.other_player() ).is_index_set(idx) { return Some(Piece::Pawn) }
        None
    }
    
    /// A helper for digging into the ply structure to touch the right pieces.
    pub fn get_mut_piece(&mut self, color: Color, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[color as usize][piece as usize]
    }

    /// A helper for digging into the ply structure to touch the right pieces.
    pub fn get_piece(&self, color: Color, piece: Piece) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    /// TODO: Prevent castling if a piece is attacking the intervening squares

    /// True if the current player both has the right to castle long and the ability.
    pub fn can_castle_long(&self) -> bool {
        match self.current_player() {
            Color::WHITE => { 
                self.meta.white_castle_long && (self.occupancy() & bitboard!("b1", "c1", "d1")).is_empty()
            }
            Color::BLACK => { 
                self.meta.black_castle_long && (self.occupancy() & bitboard!("b8", "c8", "d8")).is_empty()
            }
        }
    }
    
    /// True if the current player both has the right to castle short and the ability.
    pub fn can_castle_short(&self) -> bool {
        match self.current_player() {
            Color::WHITE => { 
                self.meta.white_castle_short && (self.occupancy() & bitboard!("f1", "g1")).is_empty()
            }
            Color::BLACK => { 
                self.meta.black_castle_short && (self.occupancy() & bitboard!("f8", "g8")).is_empty()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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