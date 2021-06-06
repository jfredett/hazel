#![allow(non_snake_case)]

use crate::constants::Piece;

///! This module defines a compact representation of chess moves from a given ply.
///!
///! Note on the name of this module. Ideally, this would be named 'move', like the struct it
///! defines, but alas, we are limited by rust reserving the `move` keyword for silly things like
///! memory safety or something.
///!

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Move(u16);

const SOURCE_IDX_MASK   : u16   = 0b111111_000000_0_000;
const SOURCE_IDX_SHIFT  : usize = 10;
const TARGET_IDX_MASK   : u16   = 0b000000_111111_0_000;
const TARGET_IDX_SHIFT  : usize = 4;
const PROMOTE_BIT_MASK  : u16   = 0b000000_000000_1_000;
const PROMOTE_BIT_SHIFT : usize = 3;
const METADATA_MASK     : u16   = 0b000000_000000_0_111;

bitflags! {
    pub struct MoveType: u16 {
        const CHECK = 0b100;
        const CAPTURE = 0b010;
        const ATTACK = 0b001;
    }
}

impl MoveType {
    /// True if the metadata encodes a check
    pub fn is_check(&self) -> bool { self.contains(MoveType::CHECK) }
    /// True if the metadata encodes a capture
    pub fn is_capture(&self) -> bool { self.contains(MoveType::CAPTURE) }
    /// True if the metadata encodes an attack on a piece
    pub fn is_attack(&self) -> bool { self.contains(MoveType::ATTACK) }
    /// True if the metadata is a quiet move
    pub fn is_quiet(&self) -> bool { self.bits() == 0 }
}

impl Move {
    pub fn empty() -> Move { Move { 0: 0 } }

    /// Creates a move from a given source and target index, 
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0b000);
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    /// ```
    pub fn from(source: u16, target: u16, is_promotion: bool, metadata: u16) -> Move { 
        let is_promote = if is_promotion { 1 } else { 0 };
        let move_val = source << SOURCE_IDX_SHIFT
                     | target << TARGET_IDX_SHIFT
                     | is_promote << PROMOTE_BIT_SHIFT
                     | metadata;
        Move { 0: 
            move_val 
        } 
    }
    
    /// Gets the source index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.source_idx(), 0o13);
    /// ```
    pub fn source_idx(&self) -> u16 { (self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT }
    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.target_idx(), 0o33);
    /// ```
    pub fn target_idx(&self) -> u16 { (self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT }
    /// True if the move indicates a promotion
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o63, 0o73, true, 0b011);
    /// assert!(!m1.is_promotion());
    /// assert!(m2.is_promotion());
    /// ```
    pub fn is_promotion(&self) -> bool { (self.0 & PROMOTE_BIT_MASK) > 0 }
    /// Calculates the promotion piece is there is a promotion to be done.
    /// NOTE: Will return garbage for non-promotion moves. No checking is done ahead of time.
    /// ```
    /// # use hazel::movement::*;
    /// # use hazel::constants::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o63, 0o73, true, 0b011);
    /// // assert!(m1.promotion_piece()); DON'T DO THIS! It's not a promotion so this is misinterpreting the union type.
    /// assert_eq!(m2.promotion_piece(), Piece::Queen);
    /// ```
    pub fn promotion_piece(&self) -> Piece { Piece::from(self.0 & METADATA_MASK) }
    /// Interprets the metadata bits when the piece is not a promotion. Use the provided `is_` functions
    /// on MoveType to interpret the data.
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o13, 0o33, false, 0b100);
    /// assert!(m1.move_metadata().is_quiet());
    /// assert!(m2.move_metadata().is_check());
    /// ```
    pub fn move_metadata(&self) -> MoveType { MoveType::from_bits(self.0 & METADATA_MASK).unwrap() }

}

/*

Idea:

A move == 16 bits broken up as follows:

3b:SourceRank | NOTE: Maybe use 6 bits and do source index -> target index?
3b:SourceFile | 
3b:TargetRank |
3b:TargetFile |
1b:Black's turn? (1 = black, 0 = white)
1b:Check? (1 = true)
1b.Capture? (1 = true)
1b:Attack? (1 = true) // true if this move would result in a new attack on one of your opponents pieces

// TODO: How do we handle promotions?
// -- perhaps restructure the metadata section to not be bitflags but rather an enum. Gets you 16
// flags instead of 4. Should be enough for representing CCA, as well as promotion to any of the 4 pieces.
// Structure would be like:
/*
// takes 2 bits
enum MoveType {
    Check = 3, 
    Capture = 2, 
    Attack = 1, 
    Quiet = 0
}

// 3 bits
enum Promotion {
    Queen = 4,
    Rook = 3 ,
    Bishop = 2,
    Knight = 1,
    None = 0
}

Also, not sure I actually need to know who's turn it is. That's for the ply to care about?

Maybe I just need to drop the movetype stuff, or reduce it to a single bit of "forcing move"? Or even just "Is it a check?" We should always consider those lines no matter what, for sure.

----

Here's an idea:

6b:SourceIdx
6b:TargetIdx
1b:Is Promotion?
{2b: If bit 13 is set, this is the promotion piece (K, B, R, Q = 2 bits)
{  : If bit 13 is not set, this is the attack metadata (Check, Capture, Attack = 2 bits)
1b: spare
*/

So 1. d4 d5, 2. Bf4 Nf3 would be:

[0b0000_011_011_011_010] => [0b0000_0110_1101_1010] => 0x06DA
[0b0000_011_101_011_110] => [0b0000_0111_0101_1110] => 0x085E
[0b0000_101_011_010_001] => [0b0000_1010_1101_0001] => 0x0AD1
[0b0000_101_101_110_111] => [0b0000_1011_0111_0111] => 0x0B88

The four metadata bits at the end are worth explaining.

There are, basically, 4 kinds of moves in chess. In order of how 'forcing' they are:

1. Checks
2. Captures
3. Attacks
4. All other moves ("Quiet" moves)

A check checks the king. It may be a mating move, it may not be, we don't care.

A capture captures a piece and is the second most forcing move because failure to respond in kind
would result in a material imbalance.

An attack adds an attacker to a piece. Attacking may provoke the need to defend the piece either
positionally or tactically. Positional defenses include blocking with another piece, retreating the
piece under attack, or creating a greater positional threat elsewhere ("Danger Levels" as IM Rozman
puts it)

All other so-called 'quiet' moves are moves which provide indirect benefit, either opening the
position to new opportunities. Denying opportunities to your opponent, or resolving positional
problems (e.g., pins, fork arrays, infiltrations, etc).

By tagging the moves with this data we can easily sort our list in order of most forcing.
Occasionally moves will fulfill two of these definitions so we want to inspect those moves first.
So as we generate moves we assign this metadata and sort the resulting array of moves so that
when evaluation comes, we are looking at the most forcing lines first.

When we evaluate moves, the evalator will chose several lines and run a depth-first 'simulation' 
playing 'random' (but weighted towards these forcing lines) moves 

---

After building this up, the next thing to do is a move generator, this should be a function which
takes a ply and produces a list of moves, I'm debating about dynamically allocating that list
(really more like shrinking it and handing out pointers to the region of memory). 

The movegen will need to understand attacks and stuff so that we can get those 4 bits in place.

If the move generator basically just has a big ass array, and then runs a couple threads which it hands
plies too and says "Enumerate all the moves", then evaluators and shit can just hold a reference into that
memory and have a RO copy of the list of moves. When the last consumer gives up the pointer, we free that
memory and re-use it.

The movegen is it's own little actor that way, so we can tell it to happily start shitting out moves while 
other things happen (or turn it off if ponder is off).

First version though can just be a vec of moves.
*/