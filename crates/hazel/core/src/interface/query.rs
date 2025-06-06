use crate::{interface::Alteration, occupant::Occupant, position_metadata::PositionMetadata, square::*};

/// implementing Query states that the implementor can provide the occupant of a square on the
/// board using standard 'Square' notation type.
pub trait Query {
    fn get(&self, square: impl Into<Square>) -> Occupant;

    /// not every Query implementer will have metadata, that's okay, but if we have it we want to
    /// be able to use it.
    fn try_metadata(&self) -> Option<PositionMetadata> {
        None
    }

    fn is_empty(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_empty()
    }

    fn is_occupied(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_occupied()
    }
}

pub fn to_alterations<Q>(board: &Q) -> impl Iterator<Item = Alteration> where Q : Query {
    let mut ret = vec![ Alteration::Clear];

    ret.extend(
        Square::by_rank_and_file()
           .filter(|s| board.is_occupied(s))
           .map(|s| Alteration::place(s, board.get(s)) )
    );

    if let Some(metadata) = board.try_metadata() {
        let metadata_information : Vec<Alteration> = metadata.into_information();
        ret.extend(metadata_information);
    }

    ret.into_iter()
}

pub fn to_fen_position(board: &impl Query) -> String {
    let mut f = String::default();
    let mut empty = 0;

    for s in Square::by_rank_and_file().downward() {
        let occ = board.get(s);
        if matches!(occ, Occupant::Empty) {
            empty += 1
        } else {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push_str(&occ.to_string());
        }

        if s.file() == 7 && s != A8 {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push('/');
        }
    }

    f.pop(); // remove the last slash

    if let Some(meta) = board.try_metadata() {
        f.push(' ');
        f.push_str(&meta.to_string());
    }

    f
}



