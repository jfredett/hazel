use std::vec::IntoIter;

use crate::types::Piece;

use super::*;

impl Square {
    /// # Absolute Movements
    /// These movements always move from the perspective of the White player, but aren't tied to
    /// any color, so 'up' always means 'increase the rank', etc.
    ///
    /// Sailing terms are used where necessary to avoid collisions with the marching terms used for
    /// Relative Movement
    ///
    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(A1.up(), Some(A2));
    /// assert_eq!(A8.up(), None);
    /// ```
    pub const fn up(&self) -> Option<Self> {
        if self.rank() == 7 {
            None
        } else {
            Some(Self(self.0 + 8))
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(A8.down(), Some(A7));
    /// assert_eq!(A1.down(), None);
    /// ```
    pub const fn down(&self) -> Option<Self> {
        if self.rank() == 0 {
            None
        } else {
            Some(Self(self.0 - 8))
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(H1.starboard(), None);
    /// assert_eq!(A1.starboard(), Some(B1));
    /// ```
    pub const fn starboard(&self) -> Option<Self> {
        if self.file() == 7 {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(A1.port(), None);
    /// assert_eq!(H1.port(), Some(G1));
    /// ```
    pub const fn port(&self) -> Option<Self> {
        if self.file() == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }


    /// # Relative Movements
    ///
    /// These movements are relative to the color of the piece moving. For example, 'forward' means
    /// "move in the direction the pawns of the given color move".
    ///
    /// I use marching terms here. Each command is the command to march in a particular direction.

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(A1.forward(&Color::WHITE), Some(A2));
    /// assert_eq!(H8.forward(&Color::WHITE), None);
    /// assert_eq!(A1.forward(&Color::BLACK), None);
    /// assert_eq!(H8.forward(&Color::BLACK), Some(H7));
    /// ```
    pub const fn forward(&self, color: &Color) -> Option<Self> {
        match color {
            Color::WHITE => {
                self.up()
            },
            Color::BLACK => {
                self.down()
            }
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(A1.backward(&Color::WHITE), None);
    /// assert_eq!(H8.backward(&Color::WHITE), Some(H7));
    /// assert_eq!(A1.backward(&Color::BLACK), Some(A2));
    /// assert_eq!(H8.backward(&Color::BLACK), None);
    /// ```
    pub const fn backward(&self, color: &Color) -> Option<Self> {
        match color {
            Color::WHITE => {
                self.down()
            },
            Color::BLACK => {
                self.up()
            }
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(A1.left(&Color::WHITE), None);
    /// assert_eq!(H1.left(&Color::WHITE), Some(G1));
    /// assert_eq!(A1.left(&Color::BLACK), Some(B1));
    /// assert_eq!(H1.left(&Color::BLACK), None);
    /// ```
    pub const fn left(&self, color: &Color) -> Option<Self> {
        match color {
            Color::WHITE => {
                self.port()
            },
            Color::BLACK => {
                self.starboard()
            }
        }
    }


    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(H1.right(&Color::WHITE), None);
    /// assert_eq!(A1.right(&Color::WHITE), Some(B1));
    /// assert_eq!(H1.right(&Color::BLACK), Some(G1));
    /// assert_eq!(A1.right(&Color::BLACK), None);
    /// ```
    pub const fn right(&self, color: &Color) -> Option<Self> {
        match color {
            Color::WHITE => {
                self.starboard()
            },
            Color::BLACK => {
                self.port()
            }
        }
    }

    /// # Derived Absolute Movements
    ///
    /// It is helpful to have diagonal movements considered, especially in const time.
    ///
    /// NOTE: I had to manually reimplement `and_then` from the stdlib, I didn't investigate
    /// deeply, but I believe that it probably could be constant time with the relevant features
    /// enabled, so perhaps someday these would be simplified by juditious use of `and_then`.
    ///
    /// Here I use sailing terms for the directions, so I don't class with the marching terms.
    ///
    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(D1.port_quarter(), None);
    /// assert_eq!(D4.port_quarter(), Some(C3));
    /// ```
    pub const fn port_quarter(&self) -> Option<Self> {
        match self.down() {
            None => None,
            Some(square) => square.port()
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(D4.starboard_quarter(), Some(E3));
    /// assert_eq!(D1.starboard_quarter(), None);
    /// ```
    pub const fn starboard_quarter(&self) -> Option<Self> {
        match self.down() {
            None => None,
            Some(square) => square.starboard()
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(D8.port_bow(), None);
    /// assert_eq!(D4.port_bow(), Some(C5));
    /// ```
    pub const fn port_bow(&self) -> Option<Self> {
        match self.up() {
            None => None,
            Some(square) => square.port()
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// assert_eq!(D4.starboard_bow(), Some(E5));
    /// assert_eq!(D8.starboard_bow(), None);
    /// ```
    pub const fn starboard_bow(&self) -> Option<Self> {
        match self.up() {
            None => None,
            Some(square) => square.starboard()
        }
    }

    /// # Derived Relative Movements
    ///
    /// Similarly for the relative movements, it is helpful to have the diagonal movements
    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(H8.left_oblique(&Color::BLACK), None);
    /// assert_eq!(D5.left_oblique(&Color::BLACK), Some(E4));
    /// assert_eq!(D4.left_oblique(&Color::WHITE), Some(C5));
    /// assert_eq!(A1.left_oblique(&Color::WHITE), None);
    /// ```
    pub const fn left_oblique(&self, color: &Color) -> Option<Self> {
        match self.forward(color) {
            None => None,
            Some(square) => square.left(color)
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(H1.right_oblique(&Color::WHITE), None);
    /// assert_eq!(D4.right_oblique(&Color::WHITE), Some(E5));
    /// assert_eq!(D4.right_oblique(&Color::BLACK), Some(C3));
    /// assert_eq!(A1.right_oblique(&Color::BLACK), None);
    /// ```
    pub const fn right_oblique(&self, color: &Color) -> Option<Self> {
        match self.forward(color) {
            None => None,
            Some(square) => square.right(color)
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(H1.left_rear_oblique(&Color::BLACK), None);
    /// assert_eq!(D4.left_rear_oblique(&Color::BLACK), Some(E5));
    /// assert_eq!(D4.left_rear_oblique(&Color::WHITE), Some(C3));
    /// assert_eq!(A1.left_rear_oblique(&Color::WHITE), None);
    /// ```
    pub const fn left_rear_oblique(&self, color: &Color) -> Option<Self> {
        match self.backward(color) {
            None => None,
            Some(square) => square.left(color)
        }
    }

    /// ```
    /// # use hazel::notation::*;
    /// # use hazel::types::Color;
    /// assert_eq!(H1.right_rear_oblique(&Color::WHITE), None);
    /// assert_eq!(D4.right_rear_oblique(&Color::WHITE), Some(E3));
    /// assert_eq!(D4.right_rear_oblique(&Color::BLACK), Some(C5));
    /// assert_eq!(A1.right_rear_oblique(&Color::BLACK), None);
    /// ```
    pub const fn right_rear_oblique(&self, color: &Color) -> Option<Self> {
        match self.backward(color) {
            None => None,
            Some(square) => square.right(color)
        }
    }

    /// Piece Moves
    /// TODO: I'm pretty sure this could be const.

    /// Return all the ways a piece of the given color _could have arrived_ on this square. e.g.,
    /// the White Pawn Unmove of D4 is C3, D2, D3, and E3. (The squares from which a white pawn
    /// could arrive on the square).
    /// BUG : 14-Nov-2024 2336
    /// ??? I truly don't understand why this type is different than the moves_for. I am hesitant
    /// to say this is Rust's fault, but I can't see why this should infer two different types. 
    /// rustc 1.84.0-nightly (b91a3a056 2024-11-07)
    /// I don't think it's rust, it's almost certainly me, but just in case this typechecks
    /// differently in the future I'll know I wasn't crazy.
    pub fn unmoves_for(&self, piece: &Piece, color: &Color) -> IntoIter<Self> {
        if piece != &Piece::Pawn {
            // The only interesing case is the pawn.
            return self.moves_for(piece, color).collect::<Vec<_>>().into_iter();
        }

        let mut ret = vec![];

        // HACK: I think this should be a method, probably on Color, but the name is taken for a
        // bitboard function that does the same thing. I don't want to break the generator as I'll
        // need to take it apart soon, so for now I'll tolerate a hack here.
        let pawn_rank = match color {
            Color::WHITE => 1,
            Color::BLACK => 6,
        };

        if let Some(sq) = self.backward(color) {
            if let Some(double_push_sq) = sq.backward(color) {
                if double_push_sq.rank() == pawn_rank {
                    ret.push(double_push_sq);
                }
            }
            ret.push(sq);
        }
        if let Some(sq) = self.left_rear_oblique(color) { ret.push(sq); }
        if let Some(sq) = self.right_rear_oblique(color) { ret.push(sq); }

        ret.into_iter()
    }

    /// Return all the moves that the Piece of the given Color could make.
    pub fn moves_for(&self, piece: &Piece, color: &Color) -> impl Iterator<Item=Self> {
        let mut ret = vec![];
        match piece {
            Piece::Pawn => {
                let sign = match color {
                    Color::WHITE => 1,
                    Color::BLACK => -1,
                };
                for (dx, dy) in &[(0, 1), (0, 2), (1, 1), (-1, 1)] {
                    let x = self.file() as isize + dx;
                    let y = sign * self.rank() as isize + dy;
                    if (0..8).contains(&x) && (0..8).contains(&y) {
                        ret.push(Square((y * 8 + x) as usize));
                    }
                }
            },
            Piece::Knight => {
                for (dx, dy) in &[(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)] {
                    let x = self.file() as isize + dx;
                    let y = self.rank() as isize + dy;
                    if (0..8).contains(&x) && (0..8).contains(&y) {
                        ret.push(Square((y * 8 + x) as usize));
                    }
                }

            },
            Piece::Bishop => {
                for (dx, dy) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let mut x = self.file() as isize + dx;
                    let mut y = self.rank() as isize + dy;
                    while (0..8).contains(&x) && (0..8).contains(&y) {
                        ret.push(Square((y * 8 + x) as usize));
                        x += dx;
                        y += dy;
                    }
                }
            },
            Piece::Rook => {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let mut x = self.file() as isize + dx;
                    let mut y = self.rank() as isize + dy;
                    while (0..8).contains(&x) && (0..8).contains(&y) {
                        ret.push(Square((y * 8 + x) as usize));
                        x += dx;
                        y += dy;
                    }
                }
            },
            Piece::Queen => {
                ret.extend(self.rook_moves());
                ret.extend(self.bishop_moves());
            },
            Piece::King => {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    let x = self.file() as isize + dx;
                    let y = self.rank() as isize + dy;
                    if (0..8).contains(&x) && (0..8).contains(&y) {
                        ret.push(Square((y * 8 + x) as usize));
                    }
                }
            },
        };
        ret.into_iter()
    }

    /// All possible moves for a pawn of the given color, regardlessof legality, but respecting the
    /// edges of the board.
    pub fn pawn_moves_for(&self, color: &Color) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::Pawn, color)
    }

    /// All possible moves for a knight, regardless of legality, but respecting the edges of the
    /// board.
    ///
    /// NOTE: Knight Moves are isomorphic by color, unlike pawns.
    pub fn knight_moves(&self) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::Knight, &Color::WHITE)
    }

    /// All possible moves for a bishop, regardless of legality, but respecting the edges of the
    /// board.
    pub fn bishop_moves(&self) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::Bishop, &Color::WHITE)
    }

    /// All possible moves for a rook, regardless of legality, but respecting the edges of the
    /// board.
    pub fn rook_moves(&self) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::Rook, &Color::WHITE)
    }

    /// All possible moves for a queen, regardless of legality, but respecting the edges of the
    /// board.
    pub fn queen_moves(&self) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::Queen, &Color::WHITE)
    }

    /// All possible moves for a king, regardless of legality, but respecting the edges of the
    /// board.
    pub fn king_moves(&self) -> impl Iterator<Item=Self> {
        self.moves_for(&Piece::King, &Color::WHITE)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use crate::types::Bitboard;
    use crate::types::pextboard;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct NonEdgeSquare(Square);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct EdgeSquare(Square);

    const EDGE_SQUARES: [Square; 28] = [
        A1, A2 ,A3, A4, A5, A6, A7, A8,
        H1, H2, H3, H4, H5, H6, H7, H8,
        B1, C1, D1, E1, F1, G1,
        B8, C8, D8, E8, F8, G8,
    ];

    impl Arbitrary for NonEdgeSquare {
        fn arbitrary(g: &mut Gen) -> Self {
            let rank = usize::arbitrary(g) % 6;
            let file = usize::arbitrary(g) % 6;
            let index = (1 + rank) * 8 + (1 + file);
            NonEdgeSquare(Square(index))
        }
    }

    impl Arbitrary for EdgeSquare {
        fn arbitrary(g: &mut Gen) -> Self {
            // A1-A8, H1-H8, B1-G1, B8-G8 = 8 + 8 + 6 + 6 = 28
            let edge_idx = usize::arbitrary(g) % 28;

            EdgeSquare(EDGE_SQUARES[edge_idx])
        }
    }

    #[quickcheck]
    fn up_works(square: Square) -> bool {
        if square.rank() == 7 {
            square.up().is_none()
        } else {
            square.up() == Some(Square(square.0 + 8))
        }
    }

    #[quickcheck]
    fn down_works(square: Square) -> bool {
        if square.rank() == 0 {
            square.down().is_none()
        } else {
            square.down() == Some(Square(square.0 - 8))
        }
    }

    #[quickcheck]
    fn starboard_works(square: Square) -> bool {
        if square.file() == 7 {
            square.starboard().is_none()
        } else {
            square.starboard() == Some(Square(square.0 + 1))
        }
    }

    #[quickcheck]
    fn port_works(square: Square) -> bool {
        if square.file() == 0 {
            square.port().is_none()
        } else {
            square.port() == Some(Square(square.0 - 1))
        }
    }

    #[quickcheck]
    fn forward_works(square: Square, color: Color) -> bool {
        match color {
            Color::WHITE => {
                if square.rank() == 7 {
                    square.forward(&color).is_none()
                } else {
                    square.forward(&color) == Some(Square(square.0 + 8))
                }
            },
            Color::BLACK => {
                if square.rank() == 0 {
                    square.forward(&color).is_none()
                } else {
                    square.forward(&color) == Some(Square(square.0 - 8))
                }
            }
        }
    }

    #[quickcheck]
    fn backward_works(square: Square, color: Color) -> bool {
        match color {
            Color::WHITE => {
                if square.rank() == 0 {
                    square.backward(&color).is_none()
                } else {
                    square.backward(&color) == Some(Square(square.0 - 8))
                }
            },
            Color::BLACK => {
                if square.rank() == 7 {
                    square.backward(&color).is_none()
                } else {
                    square.backward(&color) == Some(Square(square.0 + 8))
                }
            }
        }
    }

    #[quickcheck]
    fn left_works(square: Square, color: Color) -> bool {
        match color {
            Color::WHITE => {
                if square.file() == 0 {
                    square.left(&color).is_none()
                } else {
                    square.left(&color) == Some(Square(square.0 - 1))
                }
            },
            Color::BLACK => {
                if square.file() == 7 {
                    square.left(&color).is_none()
                } else {
                    square.left(&color) == Some(Square(square.0 + 1))
                }
            }
        }
    }

    #[quickcheck]
    fn right_works(square: Square, color: Color) -> bool {
        match color {
            Color::WHITE => {
                if square.file() == 7 {
                    square.right(&color).is_none()
                } else {
                    square.right(&color) == Some(Square(square.0 + 1))
                }
            },
            Color::BLACK => {
                if square.file() == 0 {
                    square.right(&color).is_none()
                } else {
                    square.right(&color) == Some(Square(square.0 - 1))
                }
            }
        }
    }

    #[quickcheck]
    fn interior_relative_square_tours_ortho(square: NonEdgeSquare, color: Color) -> bool {
        let square = square.0;
        square.forward(&color).and_then(|x| x.right(&color)).and_then(|x| x.backward(&color)).and_then(|x| x.left(&color)).unwrap() == square
    }

    #[quickcheck]
    #[ignore]
    fn interior_relative_square_tours_obliques(square: NonEdgeSquare, color: Color) -> bool {
        // I need an even_more_ interior square for this to work, since the obliques cover a 3x3
        // box, not a 2x2 box.
        let square = square.0;
        square.left_oblique(&color).and_then(|x| x.right_oblique(&color)).and_then(|x| x.right_rear_oblique(&color)).and_then(|x| x.left_rear_oblique(&color)).unwrap() == square
    }


    #[quickcheck]
    fn bitboard_moves_match_square_moves(piece: Piece, color: Color, sq: Square) -> bool {
        // Ignore Pawns, Kings, and Knights for now, so we can use the pextboard stuff
        if piece == Piece::Pawn { return true; }
        if piece == Piece::King { return true; }
        if piece == Piece::Knight { return true; }

        let square = sq.0;

        // this is a bitboard set with all the moves for a given piece
        let bbmoves = pextboard::attacks_for(piece, square, Bitboard::empty());

        sq.moves_for(&piece, &color).all(|x| bbmoves.is_set(x))
    }






}
