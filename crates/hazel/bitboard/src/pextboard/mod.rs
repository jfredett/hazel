mod nominal_attacks;
mod select_subset;

use hazel_core::{square::*, direction::Direction, piece::Piece};
use crate::bitboard::Bitboard;

use lazy_static::lazy_static;
use nominal_attacks::*;
use select_subset::*;


const BISHOP_OFFSETS: [usize; 64] = [
    0, 64, 96, 128, 160, 192, 224, 256, 320, 352, 384, 416, 448, 480, 512, 544, 576, 608, 640, 768,
    896, 1024, 1152, 1184, 1216, 1248, 1280, 1408, 1920, 2432, 2560, 2592, 2624, 2656, 2688, 2816,
    3328, 3840, 3968, 4000, 4032, 4064, 4096, 4224, 4352, 4480, 4608, 4640, 4672, 4704, 4736, 4768,
    4800, 4832, 4864, 4896, 4928, 4992, 5024, 5056, 5088, 5120, 5152, 5184,
];

const ROOK_OFFSETS: [usize; 64] = [
    0, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 20480, 22528, 23552, 24576, 25600, 26624,
    27648, 28672, 30720, 32768, 33792, 34816, 35840, 36864, 37888, 38912, 40960, 43008, 44032,
    45056, 46080, 47104, 48128, 49152, 51200, 53248, 54272, 55296, 56320, 57344, 58368, 59392,
    61440, 63488, 64512, 65536, 66560, 67584, 68608, 69632, 71680, 73728, 74752, 75776, 76800,
    77824, 78848, 79872, 81920, 86016, 88064, 90112, 92160, 94208, 96256, 98304,
];

/// 800KiB, equal to the sum of 2^i where i is all the shifts in ROOK_INDEX_MINS
pub const ROOK_TABLE_SIZE: usize = 819200 / 8;
/// 41KiB, see ROOK_TABLE_SIZE for details on how it's calculated.
pub const BISHOP_TABLE_SIZE: usize = 41984 / 8;

/// A PEXT-based magic bitboard
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct PEXTBoard<const SIZE: usize> {
    table: Box<[Bitboard; SIZE]>,
}

pub fn slow_attacks(pos: Bitboard, occupancy: Bitboard, dirs: [Direction; 4]) -> Bitboard {
    let mut out = Bitboard::empty();

    for dir in dirs {
        'next_dir: for i in 1..8 {
            let try_move = pos.shift_by(dir, i);
            if try_move.is_empty() {
                break 'next_dir;
            }

            let mov = try_move.all_set_squares()[0];
            out.set(mov);

            if !(try_move & occupancy).is_empty() {
                break 'next_dir;
            }
        }
    }

    out
}

pub fn slow_bishop_attacks(bishop_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    slow_attacks(
        bishop_pos,
        occupancy,
        [Direction::NW, Direction::SW, Direction::NE, Direction::SE],
    )
}

pub fn slow_rook_attacks(rook_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    slow_attacks(
        rook_pos,
        occupancy,
        [Direction::N, Direction::E, Direction::S, Direction::W],
    )
}

pub fn attacks_for(piece: Piece, sq: Square, blocks: Bitboard) -> Bitboard {
    let pos = Bitboard::from(sq);
    match piece {
        Piece::Rook => ROOK_PEXTBOARD._attacks_for(Piece::Rook, pos, blocks),
        Piece::Bishop => BISHOP_PEXTBOARD._attacks_for(Piece::Bishop, pos, blocks),
        Piece::Queen => {
            attacks_for(Piece::Rook, sq, blocks) | attacks_for(Piece::Bishop, sq, blocks)
        }
        _ => {
            panic!("Can only be called with Piece == Rook, Bishop, or Queen")
        }
    }
}

impl<const SIZE: usize> PEXTBoard<SIZE> {
    fn _attacks_for(&self, piece: Piece, pos: Bitboard, blocks: Bitboard) -> Bitboard {
        let sq = pos.first_index();
        let pext_mask = Self::mask_for(sq, piece);

        let key = Self::key_for(pext_mask, blocks);
        let offset = Self::offset_for(piece, sq);

        self.table[key + offset]
    }

    pub fn mask_for(sq: usize, piece: Piece) -> Bitboard {
        match piece {
            Piece::Rook => NOMINAL_ROOK_ATTACKS[sq],
            Piece::Bishop => NOMINAL_BISHOP_ATTACKS[sq],
            _ => panic!("Do not call with anything other than Rook or Bishop"),
        }
    }

    // determines the offset into the table for the given square
    fn offset_for(piece: Piece, sq: usize) -> usize {
        match piece {
            Piece::Rook => ROOK_OFFSETS[sq],
            Piece::Bishop => BISHOP_OFFSETS[sq],
            _ => {
                panic!("Don't call w/ blah blah look at the code")
            }
        }
    }

    fn key_for(pext_mask: Bitboard, blocks: Bitboard) -> usize {
        blocks.pext(pext_mask) as usize
    }

    pub fn rook() -> PEXTBoard<ROOK_TABLE_SIZE> {
        let mut board = PEXTBoard {
            table: Box::new([Bitboard::empty(); ROOK_TABLE_SIZE]),
        };
        board.initialize_piece(Piece::Rook);
        board
    }

    pub fn bishop() -> PEXTBoard<BISHOP_TABLE_SIZE> {
        let mut board = PEXTBoard {
            table: Box::new([Bitboard::empty(); BISHOP_TABLE_SIZE]),
        };
        board.initialize_piece(Piece::Bishop);
        board
    }

    fn initialize_piece(&mut self, piece: Piece) {
        for sq in 0..64 {
            let (pext_mask, entries) = Self::block_and_attack_board_for(piece, sq);
            for (blocks, attacks) in entries {
                let key = Self::key_for(pext_mask, blocks) + Self::offset_for(piece, sq);
                self.table[key] = attacks
            }
        }
    }

    fn block_and_attack_board_for(
        piece: Piece,
        sq: usize,
    ) -> (Bitboard, Vec<(Bitboard, Bitboard)>) {
        match piece {
            Piece::Rook => {
                let nom_attacks = NOMINAL_ROOK_ATTACKS[sq];
                let entries =
                    Self::calculate_block_and_attack_board_for(sq, nom_attacks, slow_rook_attacks);
                (nom_attacks, entries)
            }
            Piece::Bishop => {
                let nom_attacks = NOMINAL_BISHOP_ATTACKS[sq];
                let entries = Self::calculate_block_and_attack_board_for(
                    sq,
                    nom_attacks,
                    slow_bishop_attacks,
                );
                (nom_attacks, entries)
            }
            _ => panic!("Don't call with anything other than Rook or Bishop"),
        }
    }

    // TODO: sq -> Square
    fn calculate_block_and_attack_board_for<F>( sq: usize, nominal_attacks: Bitboard, attack_fn: F) -> Vec<(Bitboard, Bitboard)>
    where
        F: Fn(Bitboard, Bitboard) -> Bitboard,
    {
        let pos = Bitboard::from_index(sq);
        let blocker_indexes : Vec<usize> = nominal_attacks.all_set_squares().into_iter().map(|e| e.index()).collect();
        let mask_count = blocker_indexes.len();
        let mut out = vec![];

        for i in 0..2u64.pow(mask_count as u32) {
            let mut occupancy_board = Bitboard::empty();
            for idx in select_subset(i, &blocker_indexes) {
                occupancy_board.set(Square::new(idx));
            }
            let attack_board = attack_fn(pos, occupancy_board);
            out.push((occupancy_board, attack_board));
        }
        out
    }
}

lazy_static! {
    pub static ref ROOK_PEXTBOARD: PEXTBoard<ROOK_TABLE_SIZE> =
        PEXTBoard::<ROOK_TABLE_SIZE>::rook();
    pub static ref BISHOP_PEXTBOARD: PEXTBoard<BISHOP_TABLE_SIZE> =
        PEXTBoard::<BISHOP_TABLE_SIZE>::bishop();
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rooks {
        use super::*;

        #[test]
        fn offsets_for_rooks_calculate_correctly() {
            assert_eq!(PEXTBoard::<ROOK_TABLE_SIZE>::offset_for(Piece::Rook, 0), 0);
            assert_eq!(
                PEXTBoard::<ROOK_TABLE_SIZE>::offset_for(Piece::Rook, 1),
                4096
            );
            assert_eq!(
                PEXTBoard::<ROOK_TABLE_SIZE>::offset_for(Piece::Rook, 2),
                4096 + 2048
            );
            assert_eq!(
                PEXTBoard::<ROOK_TABLE_SIZE>::offset_for(Piece::Rook, 3),
                4096 + 2048 + 2048
            );
        }

        #[quickcheck]
        fn rook_pextboard_correctly_calculates_rook_attacks(
            sq_in: u64,
            blocks_in: Bitboard,
        ) -> bool {
            let pos = Bitboard::from_index((sq_in % 64) as usize);
            // make sure we aren't claiming the rook's square is already occupied
            let blocks = blocks_in & !pos;

            let slow_attacks = slow_rook_attacks(pos, blocks);
            let fast_attacks = ROOK_PEXTBOARD._attacks_for(Piece::Rook, pos, blocks);

            slow_attacks == fast_attacks
        }

        /* FIXME: Ply is deprecated, new movegen should replicate this test.
        #[test]
        fn slow_rook_attacks_kiwipete_a1_position() {
            let ply = Ply::from_fen(&String::from(POS2_KIWIPETE_FEN));
            assert_eq!(
                slow_rook_attacks(bitboard!("a1"), ply.occupancy()),
                bitboard!("a2", "b1", "c1", "d1", "e1")
            );
        }
        */

        /* FIXME: Ply is deprecated, new movegen should replicate this test.
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
            let rook_pos = board.rooks_for(Color::WHITE);
            let expected = bitboard!("e2", "e3", "e5", "e6", "b4", "c4", "d4", "f4", "g4", "h4");

            let rook_attacks = slow_rook_attacks(rook_pos, board.occupancy());

            assert_eq!(rook_attacks, expected);
        }
        */
    }

    mod bishops {

        use super::*;

        #[quickcheck]
        fn bishop_pextboard_correctly_calculates_bishop_attacks(
            sq_in: u64,
            blocks_in: Bitboard,
        ) -> bool {
            let pos = Bitboard::from_index((sq_in % 64) as usize);
            // make sure we aren't claiming the bishop's square is already occupied
            let blocks = blocks_in & !pos;

            let slow_attacks = slow_bishop_attacks(pos, blocks);
            let fast_attacks = BISHOP_PEXTBOARD._attacks_for(Piece::Bishop, pos, blocks);

            slow_attacks == fast_attacks
        }

        /* FIXME: Ply is deprecated, new movegen should replicate this test.
        #[test]
        fn correctly_calculates_bishop_attacks_via_slow_method() {
            /* A board what looks like:
             *
             * 8 k . . . . . . .
             * 7 . . . . . . . .
             * 6 . . . . . . p .
             * 5 . . . . . . . .
             * 4 . p . . B . . .
             * 3 . . . . . P . .
             * 2 . . . . . . . .
             * 1 K . . . . . . .
             *   a b c d e f g h
             *
             */
            let board = Ply::from_fen(&String::from("k7/8/6p1/8/1p2B3/5P2/8/K7 w - - 0 1"));

            // this is fine here since we know there is only 1 bishop on the board, this'd bust if there were two.
            let bishop_pos = board.bishops_for(Color::WHITE);
            let expected = bitboard!("f3", "f5", "g6", "d5", "c6", "b7", "a8", "d3", "c2", "b1");

            let bishop_attacks = slow_bishop_attacks(bishop_pos, board.occupancy());

            assert_eq!(bishop_attacks, expected);
        }
        */
    }
}
