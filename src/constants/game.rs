/// A position command that creates a 1.5ply game in the London.
pub const WHITE_LONDON_SHORT_UCI_POSITION : &str = "position startpos moves d2d4 d7d5 c1f4";
/// A position command that creates a 3ply king's pawn game
pub const WHITE_KINGS_PAWN_UCI_POSITION : &str = "position startpos moves e2e4 e7e5 g1f3 b8c6 f1c4 g8f6";
/// A position command that creates a game that quickly gets to the point where the white king can
/// castle kingside
pub const WHITE_KINGSIDE_CASTLE_UCI_POSITION : &str = "position startpos moves d2d4 h7h6 c1f4 g7g6 b1c3 f7f6 d1d3 e7e6";
/// A position command that creates a game that quickly gets to the point where the white king can
/// castle kingside
/// TODO: Finish this
pub const WHITE_QUEENSIDE_CASTLE_UCI_POSITION : &str = "position startpos moves <TODO>";

