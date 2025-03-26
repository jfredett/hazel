```
// NOTE: It's interesting to think about commutativity amongst - or more generally, the 'algebra'
// of -- these alterations. In particular if I'm trying to build a final representation of
// something and I want to vectorize that. It would be beneficial in some sense to be able to 'sum
// up' all the alterations, cancelling out whichever ones can be canceled, by commuting them around
// if needed. Turns don't matter if I only care about some specific sum of alterations.
//
// I guess what I'm saying is this feels like a monoid, maybe even something group-adjacent, but
// lacking an explicit NOOP instruction/identity.
//
// Assert/Inform have a less commutative structure though, so opposite to place/remove, which
// ultimately cancel each other out, e.g.:
//
// place P @ d4
// remove P @ d2
//
// results in a boardstate equivalent to:
//
// remove P @ d2
// place P @ d4
//
// and further
//
// place P @ d4
// remove P @ d2
// place P @ d5
// remove P @ d4
//
// is exactly equivalent to:
//
// remove P @ d2
// place P @ d5
//
// which you can find by commutting and cancelling like:
//
// remove P @ d2
// place P @ d4
// remove P @ d4
// place P @ d5
//
// Assert/Inform, however, require ordering, since informs have to follow a set of valid asserts.
//
//
// It's sort of a ring structure with an interesting 'dual' property. Assert/Inform are dual to
// each other, and not commutative, but Place/Remove are dual, but are commutative. Turns take the
// form `A(nR + mP)I` - some assertion, followed by some number of removals, then some number of
// placements, then post-turn information about the state of the game.
//
// Note that we do removals first as a convenience -- or else we'd need to store 2 pieces on the
// same square, this is still valid algebraically, though.
```

This was in the `alteration.rs` as of 26-MAR-2025, dig around in git for it's history, but an interesting little tidbit
about algebra and these little alteration guys.
