use crate::bitboard::Bitboard;
use super::Direction;


#[derive(PartialEq, Eq, Hash, Debug)]
struct Magic {
    mask : Bitboard,
    magic: u64
}

pub fn slow_rook_attacks(rook_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    let rook_idx = rook_pos.all_set_indices()[0];
    let rook_rank = rook_idx % 8;
    let rook_file = rook_idx / 8;
    
    let mut squares = vec![];
    
    for i in 1..=(8-rook_rank) {
        let try_move = rook_pos.shift_by(Direction::N, i);
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=rook_rank {
        let try_move = rook_pos.shift_by(Direction::S, i);
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=(8-rook_file) {
        let try_move = rook_pos.shift_by(Direction::W, i);
        if try_move.is_empty() { panic!("This should be unreachable") }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=rook_file {
        let try_move = rook_pos.shift_by(Direction::E, i);
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0]);
        }
    }
    
    let mut out = Bitboard::empty();
    for s in squares {
        out.set_by_index(s);
    }

    return out;
}


#[cfg(test)]
mod test {
    use crate::{bitboard, constants::Color, ply::Ply};

    use super::*;
    

    #[test]
    fn correctly_calculates_rook_attacks_via_slow_method() {

        /* A board what looks like:
         * 
         * 8 k . . . . . . .
         * 7 . . . . . . . .
         * 6 . . . . p . . .
         * 5 . . . . . . . .
         * 4 . p . . R . . .
         * 3 . . . . . . . .
         * 2 . . . . P . . .
         * 1 K . . . . . . .
         *   a b c d e f g h
         * 
         */
        let board = Ply::from_fen(&String::from("k7/8/4p3/8/1p2R3/8/4P3/K7 w - - 0 1"));
        
        // this is fine here since we know there is only 1 rook on the board, this'd bust if there were two.
        let rook_pos = board.rooks[Color::WHITE as usize];
        let expected = bitboard!("e2", "e3", "e5", "e6", "b4", "c4", "d4", "f4", "g4", "h4");
        
        let rook_attacks = slow_rook_attacks(rook_pos, board.occupancy());
        
        assert_eq!(rook_attacks, expected);
    }
}
