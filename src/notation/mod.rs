
struct Text(String);
struct Index(usize);

// It'd be cool to use From/Into for this, but they're not const traits so no dice.
#[const_trait]
trait Notation {
    fn as_index<const RUNTIME: bool>(&self) -> usize;
    fn from_index<const RUNTIME: bool>(index: usize) -> Self;
}

//impl Notation for UCI {
//    
//}
impl const Notation for Index {
    fn as_index(&self) -> usize { self.0 }
    fn from_index(index: usize) -> Index { Index(index) }
}
// impl Notation for Ambiguous {}

/*
pub const fn notation(notation: &str) -> Notation {
    todo!()
}
*/




#[cfg(test)]
mod tests {

    use crate::board::pieceboard::PieceBoard;

    use super::*;


    #[quickcheck]
    fn index_to_from_is_idempotent(index: usize) -> bool {
        // indicies should never be outside this range, and this is mostly for const-time
        // simplicity, so I'm not doing a tons of boundschecking. When future me comes to fix
        // whatever bug this causes, know that I'm not at all sorry about making this your problem.
        let index = index % 64;

        index == Index::from_index(index).as_index()
    }

    #[test]
    fn test_index_notation() {
        let mut board = PieceBoard::default();
        board.set_startpos();


    }
}
