use std::sync::Arc;

use owning_ref::OwningRef;

use crate::Alteration;

use super::{tapelike::Tapelike, Tape};

pub type TapelikeRef<'a, T : Tapelike> = OwningRef<Arc<T>, &'a [T::Item]>;

pub type TapeRef<'a> = OwningRef<Arc<Tape>, &'a [Alteration]>;
