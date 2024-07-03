#![feature(stmt_expr_attributes)]
#![cfg_attr(test, allow(unused_imports))]
// NOTE: These lints are disabled for the following reasons:
//
// 1. unusual byte groupings are helpful when notating the values of masks. See
// src/movement/mod.rs and it's masks for an example. It's much easier to see which
// bits are being masked this way, IMO.
//
// 2. There are a number of loops which require indexing into multi-dimensional arrays
// in various ways and doing rank/file math. It's very convenient to just specify the index
// and 'do it the old-fashioned way'. In theory, #enumerate() should be able to do this, but
// about 20 minutes of trying couldn't grok the trait bound error I was getting so I left it
// as is.
#![allow(clippy::unusual_byte_groupings, clippy::needless_range_loop)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

#[macro_use]
extern crate lazy_static;

extern crate either;
extern crate rand;

pub(crate) use thiserror::Error;
pub(crate) use tracing::{debug, error, info, instrument, warn};

#[cfg(test)]
pub use tracing_test;

#[macro_use]
pub mod bitboard;
pub mod constants;
pub mod game;
pub mod movement;
pub mod moveset;
pub mod pextboard;
pub mod ply;
pub mod util;

pub mod uci;

/// passes if the left is a subset of the right
#[macro_export]
macro_rules! assert_is_subset {
    ($left:expr, $right:expr) => {
        let mut missing = vec![];
        for m in $left {
            if !$right.contains(&m) {
                missing.push(m);
            }
        }

        if missing.len() > 0 {
            panic!("assertion failed, set difference: {:?}", missing);
        }
    };
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
