pub mod file;
pub mod conversion_tables;
pub mod color;
pub mod piece;
pub mod masks;
pub mod shifts;
pub mod move_tables;
pub mod magic;

#[cfg(test)] pub use test::*;
#[cfg(test)] pub mod test;

pub use conversion_tables::*;
pub use file::*;
pub use color::*;
pub use piece::*;
pub use masks::*;
pub use shifts::*;