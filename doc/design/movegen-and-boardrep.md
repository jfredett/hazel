# 5-OCT-2024, late night/early 6-OCT-2024 morning


I need to think this through.


A boardrep needs to support `make` and `unmake`, which takes a `Move` and applies it to the boardrep, producing a new
boardrep with the move made or unmade, as appropriate.

It should also support a `is_legal` which reports whether or not the move is legal given the provided metadata. It is
_not_ responsible for maintaining the metadata, it will be provided by the caller.

A boardrep is provided to a move generator. a move generator takes a boardrep and produces a list of possible moves for
every piece.

a psuedotrait `LegalMoveGenerator` can be used to indicate that the move generator produces only _legal_ moves.

a game maintains a stack of `HalfPly` objects containing the original notation of the move (WLOG UCI notation), this is
used to create a `Move` object. The Game can generate a state by turning these Moves into Alterations.

A boardrep can implement `make` and `unmake` automatically by implementing another trait, `Alterable` which requires
implementing the semantics of the `Alteration` enum. Make/Unmake becomes managing a stack of these Alterations and
applying the inverse alteration to unmake, up to the last marker. A struct like:


```rust

struct AlterationStack {
    stack: Vec<Alteration>,
    board: Box<dyn BoardRepresentation>
}

trait Alterable {
    /// Apply the alteration and return the results
    fn apply(&self, alteration: Alteration) -> Self;
}

trait BoardRepresentation {
    /// Is this current state legal given the provided metadata?
    fn is_legal(&self, metadata: Metadata) -> bool;

    /// Return a representation of the board after the move has been made.
    fn make(&mut self, move: Move) -> Self;

    /// Return a representation of the board after the move has been unmade.
    fn unmake(&mut self, move: Move) -> Self;
}

impl AlterationStack {
    fn make(&mut self, move: Move) {
        let alterations = move.to_alterations();
        for alteration in alterations {
            self.stack.push(alteration);
            self.board = self.board.apply(alteration);
        }
    }

    fn unmake(&mut self) {
        while match self.stack.pop() {
            Some(alteration) => {
                if alteration.is_done() { return }
                self.board = self.board.apply(alteration.inverse());
            }
            None => panic!("Cannot undo another move, stack is empty")
        }
    }
}
```

This is a rough sketch of the design.

It separates out the board rep to just be managing a single board state, leaving the metadata to the caller, which means
it'll be easy to swap out implementations of the boardstate in the final 'game' implementation.

I think adding a requirement that boardreps are `From<FEN> + Into<FEN>` (here meaning FEN without the metadata), would
be good to ensure I can easily convert between boardreps. This will allow Movegen to be implemented such that any
representation can ask for Movegen from a Movegen machine.

The MoveGen machine I am thinking of as a little subprocess that runs constantly, taking requests for moves from other
threads in the engine. This will allow two things. First, movegen being implemented as a whole subprocess means I can
keep it's caches warm and ready to start calculating, it'll also make the UCI stop/ponder/etc implementation a bit
easier. I plan to do something similar for the time management system. Second, part of the goal with hazel is to
distribute it across multiple machines, and by making it a subprocess, I can easily implement a proxy that forwards over
some transport to a remote machine.

Evaluation will work the same way, a `Game` will be provided with some channel to send requests over, the dispatcher
will forward the request to the appropriate subprocess, and the subprocess will send the result back to the dispatcher,
which sends it back to the game. 

MoveGen can also be split per piece/type of movegen, meaning as pieces are removed from the board, the movegen for that
piece can be disabled. This will allow for resources to reallocate.


# 6-OCT-2024, 1044

The above is a little bit all over the place, but I think I've got the notion, movegenv2 and boardrep are the same
problem, which means I can merge those two trees and work on it from that perspective. Since nothing has happened in the
boardrep branch, merging is trivial and I can just delete the other branch.

If I start by improving the way I handle boardrep, then the movegen stuff falls out. I'm going to work on the 'halfply'
model which will take the existing 'movement' code and turn those Move objects into vectors of Alterations. This will
then make the game object a natural stack (eventually tree) of half-plies that encapsulate the make/unmake
functionality, all I'll need to do is implement the Alterable trait for each boardrep, then the make/unmake functions
can be implemented generically.

Movegen then simply needs to generate potential halfplies, which in turn can be added to or removed from the game stack.

To accept variations, a 'movestack' can be implemented which is a stack with an added function 'make variation' which
tags the stack and continues adding halfplies to a new substack, which can be 'returned' from with preservation
(internally it can be a hash of stacks or something).

# 12-JAN-2025

## 1249 - movegen

I'm actually starting on this and my previous design is more or less out the window. A lot of things have changed since
the first iteration of the idea.

In particular, I think I can approach this from a much more abstract point of view, and I want to take a more
structured approach that is broadly piecewise.

I already have good code for determining attackers for sliding pieces, I want to build some lookup tables for pawns,
knights, and kings to have a broadly table-adjancent approach for everything, that should be pretty quick and also
conceptually simpler than the pile of algebra I'm currently do it. This'll generate all of the _possible_ moves quickly,
then I just need to prune out the illegal moves, which I hope should be easy enough to paralellize and make similarly
snappy.

Right now my plan is to reuse the `Witch` facility to create a `Conjuration` subsystem which takes a `Position` and returns
a `MoveSet` that contains all possible moves from that position. Internally it should support most of the verbs from the
old movegen system, and I'm hoping I can use that fact to be able to build up the DSL of possible queries I can make
against a boardstate, as that will be handy later on. These `Conjuration` subsystems would be pinned to particular
cores, be designed to have a relatively static cache footprint, and maybe even specialize to particular pieces. They'll
be combined into a single, common system that can then dynamically distribute work/manage how their laid out across the
architecture (possibly including remote instances, etc).

I think I'm coming around to the idea that `hazel` isn't really a chess engine (like `stockfish`) per se, nor is it even
a chess _database_ (like `ChessBase`), but rather a chess _ecosystem_, one that is designed to scale out efficiently and
with minimal effort. I don't like `ChessBase`, it's confusing, it's bloated, it's slow. `hazel`, by comparison, should
be quick and relatively easy to load games into, search those games, execute arbitrary queries against large subsets of
those games, and finally it should be efficient and easy to network it with other instances so that even relatively
low-resourced players can build a weapons-grade chess exploration system on a shoestring.

But ultimately it's all [just for fun](justforfun.dev).

## 1457 - movegen

Working a bit here, I think I want to add another abstraction, `PossibleMove`, this is a `Move` that _might_ work,
subject to being applied to some context, in particular it's a function:

```
PossibleMove :: Q -> Option<Move> where Q :: Query
```

This is what movegen can return. I can then fully encode the move AOT in a table for, e.g., Pawns, then movegen is a
lookup in the table to get a vector of possible moves. This vector is then provided a context and maps the move to None
if it's illegal, or Some(Move) if not, I can then flatten the resulting list and bingo bango.

The move table would then look like (for Pawns):

```
PAWN_MOVES[sq] -> [PossibleMove; 4]
```

with any off-board moves reducing to `const None` and the rest would encode, per piece, the rules to validate the move
in context, e.g., if you have this board:


```

.Q......
p.......
........
........
........
........
........
........

```

Then the `PAWN_MOVES[A7] -> [const None, A8=Q if not blocked, B8=Q if not blocked, const None]` -- for left attack,
advance, right attack, double advance. Captures would also have to be calculated late, but that should let me cache
_most_ of the movegen in tables, and ideally not super large ones at that.
