use super::*;


mod attacks;
mod piece_lookup;
mod occupancy;

pub use attacks::*;
pub use piece_lookup::*;
pub use occupancy::*;



impl Ply {
    pub fn king_for(&self, color: Color)    -> Bitboard { self.get_piece(color, Piece::King) }
    pub fn pawns_for(&self, color: Color)   -> Bitboard { self.get_piece(color, Piece::Pawn) }
    pub fn knights_for(&self, color: Color) -> Bitboard { self.get_piece(color, Piece::Knight) }
    pub fn bishops_for(&self, color: Color) -> Bitboard { self.get_piece(color, Piece::Bishop) }
    pub fn rooks_for(&self, color: Color)   -> Bitboard { self.get_piece(color, Piece::Rook) }
    pub fn queens_for(&self, color: Color)  -> Bitboard { self.get_piece(color, Piece::Queen) }

    pub fn mut_king_for(&mut self, color: Color)   -> &mut Bitboard { self.get_mut_piece(color, Piece::King) }
    pub fn mut_pawns_for(&mut self, color: Color)   -> &mut Bitboard { self.get_mut_piece(color, Piece::Pawn) }
    pub fn mut_knights_for(&mut self, color: Color) -> &mut Bitboard { self.get_mut_piece(color, Piece::Knight) }
    pub fn mut_bishops_for(&mut self, color: Color) -> &mut Bitboard { self.get_mut_piece(color, Piece::Bishop) }
    pub fn mut_rooks_for(&mut self, color: Color)   -> &mut Bitboard { self.get_mut_piece(color, Piece::Rook) }
    pub fn mut_queens_for(&mut self, color: Color)  -> &mut Bitboard { self.get_mut_piece(color, Piece::Queen) }
    
    pub fn all_pieces_for(&self, color: Color) -> Bitboard {
        self.pieces[color as usize]
            .iter()
            .fold(
                Bitboard::empty(), 
                |e, a| *a | e
            )
    }

}