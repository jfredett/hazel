use crate::board::PieceBoard;

use super::*;

impl From<FEN> for BEN {
    fn from(fen: FEN) -> Self {
        let mut ben = BEN::new();
        let mut idx = 0;
        let mut squares = Square::by_rank_and_file();

        while !squares.is_done() {
            let lower_square = squares.next().unwrap();
            let upper_square = squares.next().unwrap();

            let lower_occupant : u8 = fen.get(lower_square).into();
            let upper_occupant : u8 = fen.get(upper_square).into();

            ben.position[idx] = (lower_occupant << 4) | upper_occupant;
            idx += 1;
        }

        ben.metadata = fen.metadata();

        ben
    }
}

impl From<BEN> for FEN {
    fn from(ben: BEN) -> Self {
        let mut pb = PieceBoard::default();
        let mut idx = 0;
        let mut squares = Square::by_rank_and_file();

        while !squares.is_done() {
            let lower_square = squares.next().unwrap();
            let upper_square = squares.next().unwrap();

            let lower_occupant = ben.position[idx] >> 4;
            let upper_occupant = ben.position[idx] & 0b00001111;

            pb.alter_mut(Alteration::place(lower_square, lower_occupant.into()));
            pb.alter_mut(Alteration::place(upper_square, upper_occupant.into()));

            idx += 1;
        }

        let mut fen : FEN = pb.into();
        fen.set_metadata(ben.metadata);
        fen
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::EMPTY_POSITION_FEN;

    use super::*;

    #[test]
    fn from_fen() {
        let fen = FEN::new(EMPTY_POSITION_FEN);
        let ben : BEN = fen.clone().into();

        assert_eq!(ben.position, [0; 32]);
        assert_eq!(ben.metadata, fen.clone().metadata());
    }

    #[test]
    fn from_ben() {
        let ben = BEN::new();
        let fen : FEN = ben.into();

        assert_eq!(fen, FEN::new(EMPTY_POSITION_FEN));
    }
}
