
use super::*;

impl Ply {
    /// Produces a ply representing an empty board
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

    /// Produces a ply from the given FEN string
    /// NOTE: Very little error checking is done on the FEN string. Make sure to provide it
    /// with good input.
    pub fn from_fen(fen: &String) -> Ply {
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
                '-' => { 
                    /* we don't need to do anything, this should only ever appear alone, and means
                     * there are no castling rights for either side. */ 
                }
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

    pub fn to_fen(&self) -> String {
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

        out.push(' ');
        if self.meta.contains(Metadata::BLACK_TO_MOVE) {
            out.push('b');
        } else {
            out.push('w');
        } 

        out.push(' ');
        if self.meta.contains(Metadata::WHITE_CASTLE_SHORT) { out.push('K'); }
        if self.meta.contains(Metadata::WHITE_CASTLE_LONG)  { out.push('Q'); }
        if self.meta.contains(Metadata::BLACK_CASTLE_SHORT) { out.push('k'); }
        if self.meta.contains(Metadata::BLACK_CASTLE_LONG)  { out.push('q'); }

        out.push(' ');
        match self.en_passant {
            None => { out.push('-'); }
            // need to implement this
            Some(bb) => { 
                // If there are multiple en passant indices, then it's a broken state anyway
                let idx = bb.all_set_indices()[0];
                out.push_str(&INDEX_TO_NOTATION[idx]);
            }
        }

        out.push(' ');
        out.push_str(&self.half_move_clock.to_string());

        out.push(' ');
        out.push_str(&self.full_move_clock.to_string());

        return out;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    mod from_fen {
        use super::*;

        #[test]
        fn parses_starting_position_correctly() {
            let start_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let ply = Ply::from_fen(&start_fen);
            assert_eq!(ply, ply::test::start_position());
        }

        #[test]
        fn parses_london_position_correctly() {
            let fen = String::from("r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7");
            let ply = Ply::from_fen(&fen);
            assert_eq!(ply, ply::test::london_position());
        }
    }

    mod to_fen {
        use super::*;

        #[test]
        fn round_trips_starting_position() {
            let fen_in = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }

        #[test]
        fn round_trips_london_position() {
            let fen_in = String::from("r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7");
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }

        #[test]
        fn round_trips_position_with_en_passant() {
            let fen_in = String::from("r1bqk2r/pp2bppp/2n1pn2/3p4/1PpP1B2/2P1PN1P/P2N1PP1/R2QKB1R b KQkq b3 0 8");
            let ply = Ply::from_fen(&fen_in);
            let fen_out = ply.to_fen();
            assert_eq!(fen_in, fen_out);
        }
    }
}