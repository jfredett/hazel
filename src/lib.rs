#![feature(nll, box_syntax)]
#![cfg_attr(test, allow(unused_imports))]

pub use packed_simd::*;

#[cfg(test)]
#[macro_use] extern crate quickcheck_macros;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate bitflags;

extern crate either;
extern crate rand;

pub use rand::prelude::*;

#[macro_use] pub mod bitboard;
pub mod constants;
pub mod ply;
pub mod movement;
pub mod moveset;



/// passes if the left is a subset of the right
#[macro_export] macro_rules! assert_is_subset {
    ($left:expr, $right:expr) => (
        let mut missing = vec![];
        for m in $left {
           if !$right.contains(&m) {
                missing.push(m);
           } 
        } 
        
        if missing.len() > 0 {
            panic!("assertion failed, set difference: {:?}", missing);
        }
    );
}

/*
FIXME: This isn't working because it can't find the 'assert_is_subset!' macro even though it's _right there_. 

/// This is essentially assert_eq but doesn't care about order differences
#[macro_export] macro_rules! assert_are_equal_sets {
    ($left:expr, $right:expr) => (
        assert_is_subset!(&$left, &$right);
        assert_is_subset!(&$right, &$left);
    );
}

*/