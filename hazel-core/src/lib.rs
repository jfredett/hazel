#![feature(assert_matches, associated_type_defaults, const_for, const_trait_impl, let_chains, lock_value_accessors, new_range_api, stmt_expr_attributes)]
// The Squares being consts means that sometimes when I use them as a reference, I trigger this
// warning. I generally don't mind the temporary value being created, and in fact want it (see the
// PositionMetadata to/from u32 impls for an example).
#![allow(const_item_mutation)]
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

extern crate rand;

#[cfg(test)]
pub use tracing_test;


pub mod board;
pub mod constants;
pub mod coup;
pub mod engine;
pub mod game;
pub mod interface;
#[macro_use]
pub mod types;
