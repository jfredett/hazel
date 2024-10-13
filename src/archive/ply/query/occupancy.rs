use super::*;

impl Ply {
    // OCCUPANCY

    /// Provides a bitboard which shows the location of all squares occupied by pieces of the given
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::types::Bitboard;
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
        self.king_for(color)
            | self.queens_for(color)
            | self.rooks_for(color)
            | self.bishops_for(color)
            | self.knights_for(color)
            | self.pawns_for(color)
    }

    /// Provides a bitboard which shows the location of all squares occupied by pieces of any
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::types::Bitboard;
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
}
