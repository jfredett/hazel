
use super::*;

impl Ply {
    /// Produces a ply representing an empty board
    pub fn empty() -> Ply {
        Ply {
            pieces: [[Bitboard::empty(); 6]; 2],
            meta: Metadata::default(),
        }
    }

    /// Produces a ply from the given FEN string
    /// NOTE: Very little error checking is done on the FEN string. Make sure to provide it
    /// with good input.
    pub fn from_fen(fen: &str) -> Ply {
        // A cheap and cheerful fen parser, very little error handling
        let fen_parts : Vec<&str> = fen.split(' ').collect();
        let mut ply = Ply::empty();

        // Board setup
        let mut rank = 7;
        let mut file = 0;
        for ch in fen_parts[0].chars() {
            match ch {
                'p' => { ply.mut_pawns_for(Color::BLACK).set(rank,file);   file += 1; }
                'k' => { ply.mut_king_for(Color::BLACK).set(rank,file);    file += 1; }
                'q' => { ply.mut_queens_for(Color::BLACK).set(rank,file);  file += 1; }
                'r' => { ply.mut_rooks_for(Color::BLACK).set(rank,file);   file += 1; }
                'b' => { ply.mut_bishops_for(Color::BLACK).set(rank,file); file += 1; }
                'n' => { ply.mut_knights_for(Color::BLACK).set(rank,file); file += 1; }
                'P' => { ply.mut_pawns_for(Color::WHITE).set(rank,file);   file += 1; }
                'K' => { ply.mut_king_for(Color::WHITE).set(rank,file);    file += 1; }
                'Q' => { ply.mut_queens_for(Color::WHITE).set(rank,file);  file += 1; }
                'R' => { ply.mut_rooks_for(Color::WHITE).set(rank,file);   file += 1; }
                'B' => { ply.mut_bishops_for(Color::WHITE).set(rank,file); file += 1; }
                'N' => { ply.mut_knights_for(Color::WHITE).set(rank,file); file += 1; }
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

        // TODO: Move this part into a method on Metadata itself.
        match fen_parts[1] {
            "w" => { ply.meta.to_move = Color::WHITE; }
            "b" => { ply.meta.to_move = Color::BLACK; }
            _ => { panic!("Invalid FEN color: {}", fen); }
        };

        // castling rights
        for ch in fen_parts[2].chars() {
            match ch {
                'K' => { ply.meta.white_castle_short = true; }
                'Q' => { ply.meta.white_castle_long = true; }
                'k' => { ply.meta.black_castle_short = true;  }
                'q' => { ply.meta.black_castle_long = true;  }
                '-' => { 
                    /* we don't need to do anything, this should only ever appear alone, and means
                     * there are no castling rights for either side. */ 
                }
                _ => { panic!("Invalid FEN castling key: {}", fen); }
            }
        }

        ply.meta.set_en_passant(match fen_parts[3] {
            "-" => None,
            _ => Some(NOTATION_TO_INDEX(fen_parts[3]))
        });
        
        ply.meta.half_move_clock = fen_parts[4].parse().unwrap_or_else(|_| panic!("Invalid FEN half-move: {}", fen));
        ply.meta.full_move_clock = fen_parts[5].parse().unwrap_or_else(|_| panic!("Invalid FEN full-move: {}", fen));

        ply
    }

    pub fn to_fen(self) -> String {
        let board = self.board_buffer();
        let mut out = String::new();

        for row in board.iter().rev() {
            let mut skip = 0;
            for &c in row {
                // if there is no piece here, increment our skip counter
                if c == '.' {
                    skip += 1
                // if we have an active skip counter and the current item is a piece
                } else if skip > 0 && c != '.' {
                    // push our skipcount
                    out.push_str(&skip.to_string());
                    // push the piece
                    out.push(c);
                    // reset skip count
                    skip = 0;
                } else {
                    // no skip counter and the value is not empty, so just push it
                    out.push(c);
                }
            }
            // in the event the whole row is blank, we'll get here with an active skip, so we need to clear it now
            if skip > 0 {
                out.push_str(&skip.to_string());
            }
            out.push('/');
        }
        out.pop(); // we have an extra '/' so we need to remove it

        // TODO: move into a to_fen on Metadata
        out.push(' ');
        if self.meta.to_move.is_black() {
            out.push('b');
        } else {
            out.push('w');
        } 

        out.push(' ');
        if self.meta.white_castle_short { out.push('K'); }
        if self.meta.white_castle_long  { out.push('Q'); }
        if self.meta.black_castle_short { out.push('k'); }
        if self.meta.black_castle_long  { out.push('q'); }

        out.push(' ');
        match self.en_passant() {
            None => { out.push('-'); }
            // need to implement this
            Some(bb) => { 
                // If there are multiple en passant indices, then it's a broken state anyway
                let idx = bb.first_index();
                out.push_str(INDEX_TO_NOTATION[idx]);
            }
        }

        out.push(' ');
        out.push_str(&self.meta.half_move_clock.to_string());

        out.push(' ');
        out.push_str(&self.meta.full_move_clock.to_string());

        out
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    use super::tests::*;

    mod from_fen {
        use super::*;

        #[test]
        fn parses_starting_position_correctly() {
            let start_fen = String::from(START_POSITION_FEN);
            let ply = Ply::from_fen(&start_fen);
            assert_eq!(ply, start_position());
        }

        #[test]
        fn parses_london_position_correctly() {
            let fen = String::from(LONDON_POSITION_FEN);
            let ply = Ply::from_fen(&fen);
            assert_eq!(ply, london_position());
        }
    }

    mod to_fen {
        use super::*;

        #[test]
        fn round_trips_starting_position() {
            let fen_in = String::from(START_POSITION_FEN);
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }

        #[test]
        fn round_trips_london_position() {
            let fen_in = String::from(LONDON_POSITION_FEN);
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }

        #[test]
        fn round_trips_position_with_en_passant() {
            let fen_in = String::from(EN_PASSANT_POSITION_FEN);
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }
    }
}