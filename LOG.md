# 21-JUN-2024

## 1610

Working on getting this fixed up and functional, I updated dependencies, cleaned up some syntax
stuff, got a `flake` set up and `rustfmt` configured and running. I'll add precommits at some point
maybe.

I'm working on the pgn parsing issue, it appears to be failing to generate blocking moves for check.
It gets stuck on 26. .. Rf7 in the example fixture at tests/fixtures/no-variations.pgn. The `perft`
test is also over-counting the number of moves, so I need to figure out why it isn't seeing the rook
move.

I'm going to check out [bugstalker](https://github.com/godzie44/BugStalker) to see if it helps suss
out what's happening.

## 2115

I found the issue. I think my movegen was more or less fine? I added some logic about looking for
'blocking squares', but I don't actually know if it's necessary. I'll have to come back and test it
when I get to the refactor stage. Right now I've chased down a different bug which appears to be due
to `Game::from_fen/1` not calculating metadata from the FEN string. It ultimately delegates to
`Ply::from_fen/1`, which appears to incorrectly grant castling rights to black, which is incorrect.
I suspect this is because `Metadata::default()` grants castling rights, but in the event I see the
`-` value, I don't do anything, which means it breaks.

## 2133

I got everything pushed up onto `main`, switched the branch over, and need to just do a few more
cleanup tasks before I can set the repo to open.

## 2147

Added a LICENSE (AGPLv3), README, and uncommented the next tier of `perft`, which is failing due to
unimplemented `EP_CAPTURE` in the `src/ply/make.rs` module.

For now, it's good enough to go live I think. I still need to chase out some dependabot warnings,
but that shouldn't be too hard

# 22-JUN-2024

## 0143

Worked a bit on EP_CAPTURE, but it appears that the `unmake` method marked `todo!()` that I started
implementing isn't actully getting called in my test that explicitly exercises the unmake function
for this. It seems to be failing further up the chain, so I managed to stumble on a different bug
while trying to get perft working up to 6-ply.

## 1221

I think the issue may be that it's not properly recording that the previous move was an en passant?

## 1318

I've got it figured out, I just needed to do some calculation to get all the pieces moved correctly.
I need to add another couple tests for en passant on the edge files, one for an EP by black on
white, and a few other cases, but I suspect `perft` will reveal if there are any such issues.

## 2117


```
2024-06-22T21:48:00.245032Z DEBUG perft_start_position_to_depth_6:perft{depth=5}:perft{depth=4}:perft{depth=3}:perft{depth=2}:perft{depth=1}: hazel::game: 
8 | r . b q k b n r
7 | p . p p p p p p
6 | n P . . . . . .
5 | . . . . . . . .
4 | . . . . . . . .
3 | . . . . . . . .
2 | . P P P P P P P
1 | R N B Q K B N R
    a b c d e f g h
    Black to play

2024-06-22T21:48:00.245161Z DEBUG perft_start_position_to_depth_6:perft{depth=5}:perft{depth=4}:perft{depth=3}:perft{depth=2}:perft{depth=1}: hazel::game: 
8 | r . b q k b n r
7 | p . p p p p p p
6 | n P . . . . . .
5 | p P . . . . . .
4 | . . . . . . . .
3 | . P . . . . . .
2 | . . P P P P P P
1 | R N B Q K B N R
    a b c d e f g h
    Black to play
```

This is where the bug is introduced. In the first Ply, White plays axb6, and in the second Ply, the
`unmake` function has been called but did not properly accomplish the unmove. By the time we see it,
White has played b3, but the board is in an incorrect state because the en passant didn't unwind
properly.

I'm not 100% sure what I did wrong, but I think the answer is probably to add a bunch more tests.

One thing I could do is have each ply have a pointer to a 'previous' ply, rather than maintaining a
full history in the ply itself. This would trivialize the `unmake` function, but at the cost of way
more memory use (I think).

I think the first step is to analyze the `unmake` side of the en passant, and make sure it works
consistently. I'm looking forward to when I have the movegen side of this working correctly, so I
can refactor it to be less of a giant megamethod, I think at least some of my pain is found there.

# 30-JUN-2024

## 0102

Been working on the EP_CAPTURE issue, I ended up just deleting the implementation, walking away for
a bit, then reimplementing. I ended up doing it a slightly different way than the inital bit
mangling trick I was trying, and I think I'm just going to leave it as is for now. I think the bit
mangling will be faster and I was probably just doing it wrong, but I can leave the optimization for
another day.

I added some better error output for the unrecoverable error branch, and managed to get the `perft`
test running (but getting a different count of positions than it should).

I need to set up some kind of integration test w/ a known-good engine. I think I might try switching
off the movegen for a bit and work on the UCI implementation and maybe a little TUI for debugging. I
could then ostensibly get it and stockfish talking to each other, and it could then start perfting
and having SF verify the results, so I can start to see the game state that led to it overcounting.

In any case, for now, EP_CAPTURE I think is working, and I won't really be able to track down the
bug until I build up some better tools for debugging.

## 1838

A rough design:

Hazel: The main thread, spawns:
    - Grid: A list of engine instances, defaulting to Hazel, but allowing arbitrary UCI connections
      to other engines.
    - UI: The UI thread
    - Race Control: Which sends commands between engines / the UI, may be part of the main Hazel thread
      instead of it's own thing, not really sure.

The idea is to let Hazel have a sort of 'tournament management' feature, where it can run multiple
different engines and manage games between them, and also allow for deep integration tests as above.

The Engine Instance is a UCI Socket for communicating to the engine, which Hazel manages and the UI
allows selection between, messages are proxied down to the actual engine instance, which is just
running it's own UCI client reading from the same socket.

# 1-JUL-2024

## 2242

Got the Ratatui UI roughed in; also sketched out a plan for the debugging
interface/engine-tournament-manager thing. I'm off on a rabbit trail, but it seems fun so probably
worth doing. 

The first thing I want to do is get some sort of widget built to display a chessboard, I don't think
that should be too difficult, since I've got half an implementation in the Debug impl for Ply.

I can't say I know enough about `ratatui` so far to have an opinion, but I'll say for sure that the
markup-in-Rust thing is not my favorite. Copilot actually helps a bit here, these APIs kind of suck
to remember; I frequently get simple transpotition errors where I miscapitalize or whatever. Copilot
eventually figures out what I'm trying to do and at least makes the mistakes for me, so I'm just
fixing what it fucks up instead of me making the mistake and feeling bad about it.

## 2307

Futzing more, I think it makes sense to include all the UI stuff in the library proper, and maybe
just gate it by a feature someday. I can implement widgets directly against the types in the
library, which seems nicer than having to wrap things in a newtype to widget them.

I'm definitely not quite doing it right w/ rendering, this is because I'm procrastinating reading
the rest of this tutorial.

# 3-JUL-2024

## 0941

I think the UI is in a good spot to leave for a minute while I bring in some of the components that
need the UI. The first round is a simple UCI REPL; I should be able to type raw UCI into the UI and
have it respond (probably with dummy info for now). It should expose an option "current engine" that
points to the relevant backend by name.

I also managed to re-introduce an en passant bug, I'm going to leave it for now; I started on a
refactored movegen module before I put the project down last time, I think that's probably the
'right' way to chase out these bugs, and I think having a better debugging tool will help a lot in
figuring out what the issue is.

## 1145

I've roughed in the UCI repl a bit, just the parsing stuff. I'm going to get a basic
parse-to-an-enum, then refactor to use Hazel types for Moves/etc. Those are generic enough that it
should help with forwarding UCI around but also being ergonomic enough to use directly in the UI and
for Hazel.

## 1320

Fully roughed in, no type fanciness yes, but I'll enrich the parsing as I need it. I think the next
step is to get the OCI connection started up, and then work on a backend which proxies the OCI
connection to the backend engine.

## 1335

I think I need to learn `tokio`. I keep hemming and hawing about just writing my own threadpool
thing, but I should probably learn the 'real' tool instead of rolling my own here. Time for more
tutorial reading, I guess.

Or not, [this article](https://corrode.dev/blog/async/) suggests that maybe threads are just fine. I
don't know that this needs to be such a giant thing, I'm comfortable having lots of otherwise idle
threads with connections open. I suppose maybe a better approach would be to have a
thread-per-engine, and then keep them in a pool. I suppose I could have `Grid` be abstract across
any `Engine`, and then have `Engine` be a trait which, e.g., `HazelEngine` might implement.

The trait would just expect a way to send it UCI messages and recieve them in return. Eventually I
can place another variant which supports, e.g., a protobuf-defined API or something. The API would
have a `raw_uci` message so you could always fall back to just parsing UCI, but for richer queries I
could extend it.

I think I'll skip `tokio` for now and stick with threads until it starts hurting.


## 2143

Definitely a pass on `tokio`, the docs say that I'm probably not the right case for it right now,
and if I can go with a threaded approach, go with a threaded approach, so that sounds like a good
reason to me.

I'm going to work on getting the 'Driver' part of Hazel working, that's going to be the thing that
actually takes UCI messages and plays the game. My goal is going to be to integrate the UCI parser I
just wrote with the `Game` struct, and implement whichever commands feel easy and bail on the rest.
That should be enough for me to hook hazel up to an OTS GUI, and then I can get something relatively
bug free working on a known-good GUI before working to implement my own. I can also hopefully hook
up my UI and the OTS one at the same time, so I can compare when debugging.


# 4-JUL-2024

## 1713

Been picking at this all day, managed to get Arena to connect to Hazel and for Hazel to dump it's
logs somewhere other than STDOUT. I imagine I could have the thing continue to dump logs to STDOUT
but I'm not sure if that'll cause weirdness later with the UI, so I'm just going to leave it sending
to `/tmp/hazel.log` until I have some UI in place.

I think I'm going to work towards getting it making random moves and generally implementing the rest
of UCI for now. I also want to get it so that when `debug` is turned on, the `debug` log level is
set on the subscriber, and otherwise leave it at `info`. That way I can continue to use Arena as a
debugging mechanism.

I slapped together a couple scripts for running `cargo test` and `cargo build` with warnings
suppressed. I wish this was an option on the command itself instead of a flag. I recognize the 'good
practice' of always leaving these on, but I think it's a little misguided, most of the time I
prioritize fixing the thing I'm working on, and because the error occasionally pop up in the middle
of all the warnings, it's hard to track down the error amidst them.

I did a little experimenting with `watchman` and I think it may be worth a little hackery to come up
with something that will automatically, incrementally rerun tests and give you a little grid display
+ log output of failing tests. Mostly I want it to exist, I don't know that I actually want to build
such a thing.

In any case, good progress today, hopefully I can get most of the protocol implemented and see if it
can help me debug any of the weird EP bugs I've been seeing.

# 18-SEP-2024

## 1505

Stuck on a work problem, decided to commit off some work here while thinking about it. Several
things are in flight here but I haven't been back fore a while because I simply can't bring myself
to work on UI code. I'm procrastinating even now.

I'm debating whether I want a special 'Hazel' widget that can bypass the UCI interface, or if I want
to support a sort of UCI+ interface that allows for custom commands to be passed through, so I can
just focus on using an existing UCI interface. I'm probably going to branch off and work on both,
because making decisions is hard.

# 20-SEP-2024

## 0104

I've roughed in the UI model/viewmodel layout, and also made a quick 'driver' that talks to
stockfish over my UCI protocol implementation. I also set up an implementation of Stockfish's `d`
command and more generally I think I'm going to work on building up a UCI+ style implementation for
now, with an eventual goal of supporting multiple distinct protocols and transports.

I'm already seeing the UI side of this being separable, but for the moment I think there is some
benefit to keeping it embedded, in particular for building deep analysis tooling into the engine.

The next step is to start building widgets that query against the `RaceControl` backend. I want to
try to set up threading early in the UI so I suspect the way the widget is managing state might be
subject to a lot of change.

# 24-SEP-2024

## 1932

I've got things looking a bit more operational now. I've refactored the UI a lot to be more aligned 
with the tutorial, and I've created a skeleton for the datamodel.

I also refactored some of the terminal handling stuff, and chased some bugs in the stockfish wrapper
implementation.

I started on a `mask.rs` which is going to supercede that stockfish module and be able to manage any
STDIO/unix-y process, which should help with future scaling, but it's tricky and I'm debating
whether I should just bite the `tokio` bullet or not.

Next step is to improve things by getting state tracking set up on the Hazel side, since engines
generally don't have ways to display their boardstate (stockfish being a notable exception).

I think I do have enough now to write a test that tries to 'play' random moves (generated by Hazel)
for both sides and tries to see if stockfish and hazel ever disagree (including the PGN used to get
there as well).

I did a little coverage testing and I need to get that automated, so somewhere in here there's going
to be some github action setup to do.

I think I'm close to mergable, though the UI is total trash, I'd prefer to keep developing on main,
so I'll probably merge soon, perhaps after taking a detour to chase out that damn en passant bug.


## 2121

I chased down that en passant bug, I believe it was a bad metadata provided in the unmake call, but
probably I should be calculating the 'correct' metadata from the 'unmade' moves metadata. The whole
`Ply` struct is pretty cooked, tbh.

Anyway, back to `main` I go.

# 26-SEP-2024

## 2034

I set up a `mask` util object, but this is going to require refactoring `Engine` before I can use
it. I think I can create a second trait, `AsyncEngine`, which wraps `Engine`, allowing for both.

I think I need to switch to only returning messages one at a time, rather than trying to eagerly
read. All this needs to factor into the UI as well, which is going to be a bunch of work as well.

For now, I think I can progress with the blocking version and try to get the UI 'working' to some
extent with a stockfish backend. Thus back to the UI I guess. Logging output and text-to-stdin input
next, I think.

# 27-SEP-2024

## 0145

I have foolishly stayed up too late working on this, but I've got the UI in a good spot. I think I've identified the
mistake in my approach, in particular, I'm trying to extract modules/structs too early, I should start with state living
on Hazel and extract the widgets I need as methods, then abstract the state away in bigger chunks.


# 29-SEP-2024

## 1014

I've added a 'placeholder' widget so I can work more from the top-down on the UI. I designed a UI in the `ui-sketch.md`
and I'm going to start working towards that now that I have a concept of what I want things to look like. My hope is
that I can start with the layout of mostly `placeholder` invokes. The whole layout expects a static size, and eventually
I want the UI to just be a window on a larger, statically rendered 'infinite canvas' type of thing.

I should emphasize that I am not a UI/UX guy and I'm basically making this up as I go, but I think it'll be cool.

# 1-OCT-2024

## 1739

A little idea I had to improve board management.

I want to have a single `MoveType` that is respected across boards, and to me, that really comes down to that
`Alteration` idea I used in the `Pieceboard` type. In particular, I reduced from thinking about a move as a monolithic
enum like:

```rust
pub enum MoveType {
    QUIET = 0b0000,
    DOUBLE_PAWN = 0b0001,
    SHORT_CASTLE = 0b0010,
    LONG_CASTLE = 0b0011,
    CAPTURE = 0b0100,
    EP_CAPTURE = 0b0101,
    UNUSED_1 = 0b0110,   // NOTE: unused at the moment
    ///
    /// UCI sends moves in a simplified long algebraic notation that simply specifies:
    ///
    /// 1. source square
    /// 2. target square
    /// 3. promotion piece (if any)
    ///
    /// For Hazel, more is expected of the move metadata, but calculating it requires knowing the
    /// game state. This value is used to indicate that the move metadata is ambiguous and needs to
    /// be calculated.
    /// 
    UCI_AMBIGUOUS = 0b0111,
    PROMOTION_KNIGHT = 0b1000,
    PROMOTION_BISHOP = 0b1001,
    PROMOTION_ROOK = 0b1010,
    PROMOTION_QUEEN = 0b1011,
    PROMOTION_CAPTURE_KNIGHT = 0b1100,
    PROMOTION_CAPTURE_BISHOP = 0b1101,
    PROMOTION_CAPTURE_ROOK = 0b1110,
    PROMOTION_CAPTURE_QUEEN = 0b1111,
}
```

Coupled with some location information, I can instead think of a move as entirely defined by a series of simpler
`Alteration`s to the board.

```
enum Alteration {
    Remove(Location, Color, Piece),
    Place(Location, Color, Piece),
    Comment(&'static str)
}
```

A location is a compact representation of a spot on the board, the Piece is a compact representation of a piece, since
there are 64 squares and 6 possible pieces, and 2 colors, then there these can be tightly packed into a single byte. A
move may be multiple 'alterations' e.g., a Capture a black pawn on `e4` with a Knight by White would be:

```rust
Comment("1. Ne4 ...")
Remove(e4, Black, Pawn)   // Remove the target piece, the piece metadata here is used for unmoving.
Remove(d2, White, Knight) // Remove the source piece from it's original location.
Place(e4, White, Knight)  // Place it in it's new location.
```

The `Comment` allows for some clever tricks later. This representation means we can represent the game state as a series
of trivially reversible alterations, and we can use the `Comment` to act as a checkpoint to know when we've unwound
enough. Every board representation can, ultimately, just implement some parser for these alterations, and then we can
decouple make/unmake from the board representation entirely. Movegen would query a representation of the board, generate
a bunch of `Move`s, which can then be 'compiled' as alterations.

It also means that internally, the engine can happily keep a couple different representations in various stages of the
game tree depending on what the engine might need for evaluation. It can have a pieceboard rep, perhaps, for doing one
kind of analysis where the pieceboard is convenient, and so long as it is only working on that analysis, it can keep it
up to date to the exclusion of other reps. Similarly, it can roll forward or back as needed.

This also makes it much easier to maintain UI-friendly represnetations that can benefit from the faster internal
representations. We can pick up the list of alterations and apply them unchanged.

Ultimately this boils down to a `BoardRepresentation` trait which looks like:

```rust
type Move = Vec<Alteration>;

trait BoardRepresentation {
    fn make(&mut self, alteration: Alteration);
}

pub fn unmake<BR>(board: &mut BR, alteration: Alteration) where BR: BoardRepresentation {
    board.make(!alteration);
}

trait MoveGenerator {
    // Calculate _all_ (not necessarily only legal) forward moves from the current position.
    fn moves(&self) -> Vec<Move>;
}

trait LegalMoveGenerator {
    // Calculate only legal moves from the current position.
    fn legal_moves(&self) -> Vec<Move>;
}

```

This will make it easy to differentiate between UI/Non-UI reps (e.g., whether it implements MoveGenerator or
LegalMoveGenerator). Meanwhile, `!` starts to do some heavy lifting because:

```
impl Not<Alteration> for Alteration {
    fn not(self) -> Alteration {
        match self {
            Remove(loc, color, piece) => Place(loc, color, piece),
            Place(loc, color, piece) => Remove(loc, color, piece),
            Comment(c) => Comment(c),
        }
    }
}
```

For completeness, every move type is represented:


```
Quiet(src, dest):
    src_color, src_piece = get(src)
    Place(dest, src_color, src_piece)
    Remove(dest, src_color, src_piece)

ShortCastle(color):
    if color == White:
        Place(6, White, King)
        Remove(4, White, King)
        Place(5, White, Rook)
        Remove(7, White, Rook)
    else:
        Place(62, Black, King)
        Remove(60, Black, King)
        Place(61, Black, Rook)
        Remove(63, Black, Rook)

LongCastle(color):
    if color == White:
        Place(2, White, King)
        Remove(4, White, King)
        Place(3, White, Rook)
        Remove(0, White, Rook)
    else:
        Place(58, Black, King)
        Remove(60, Black, King)
        Place(59, Black, Rook)
        Remove(56, Black, Rook)

Capture(src, dest):
    src_color, src_piece = get(src)
    Place(dest, src_color, src_piece)
    Remove(dest, src_color, src_piece)
    Remove(src, src_color, src_piece)

EnPassant(src, dest):
    src_color, src_piece = get(src)
    Place(dest, src_color, src_piece)
    Remove(dest, src_color, Pawn)
    Remove(dest, !src_color, Pawn)


Promotion(src, dest, piece):
    src_color, src_piece = get(src)
    Place(dest, src_color, piece)
    Remove(dest, src_color, src_piece)

PromotionCapture(src, dest, piece):
    src_color, src_piece = get(src)
    Place(dest, src_color, piece)
    Remove(dest, src_color, src_piece)
    Remove(dest, !src_color, piece)
```

These rely on an assumed internal 'get' function that retrieves the piece at a location, but I think that's too low
level to include in the trait, since `get` might be different depending on context and how you want to implement things.

In any case, this should (I think) make for a bit of an easier make/unmake system. It _may_ be possible to make it fully
piece/color agnostic, but I haven't thought it that far through yet.

# 3-OCT-2024

## 2049

Working on getting the UI to be 'reactive', and my heirarchy is making for a real annoying problem of having to thread
the various bits of state through the UI, it would be convenient but clumsy to have it all in a single object on Hazel,
laying out the UI a little more rigidly there would probably work, I think I could simply render to an internal
'infinite' buffer and then grab a XxY view to actually render to the screen.

I think a more elegant way, though, is to make the main UI object an async object and have it run a little message queue
that each widget can listen for events addressed to it on. So the handler would grab the event, parse it based on the
current UI state, and enqueue a message which can be seen by any widget and reacted to. This would allow for each widget
to manage all it's own internal state, just by subscribing to this message queue. Leaving the object structure to
represent the DOM, essentially.

Did I accidentally re-invent React? I don't know, just seems logical to me, I'm not a UI programmer.

For the moment I'm just going to eat crow and thread everything through, it's ugly but it will work.


# 4-OCT-2024

## 2339

While working on the UI yesterday and a bit today, I thought I was going to have the opportunity to do some extremely
cursed things with unicode variables and stuff, but alas it was not to be. I did, however, make this at one point:


```rust

let table = Table::new([
    Row::new(vec!["┌", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┐"]),
    Row::new(vec!["│", " ", "♜", " ", "│", " ", "♞", " ", "│", " ", "♝", " ", "│", " ", "♛", " ", "│", " ", "♚", " ", "│", " ", "♝", " ", "│", " ", "♞", " ", "│", " ", "♜", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│"]),
    Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
    Row::new(vec!["│", " ", "♖", " ", "│", " ", "♘", " ", "│", " ", "♗", " ", "│", " ", "♕", " ", "│", " ", "♔", " ", "│", " ", "♗", " ", "│", " ", "♘", " ", "│", " ", "♖", " ", "│"]),
    Row::new(vec!["└", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┘"]),


], Constraint::from_maxes(vec![1].repeat(33)))
.column_spacing(0)
.style(
    Style::default().bg(Color::White).fg(Color::Black)
);
```

Which is beautiful in both rendering and absolute piss-elegance. I did not want it to go to waste in /dev/null, so I
have recorded it here.


# 5-OCT-2024

## 0015

These too:

```rust

const EMPTY : &'static str = " ";

const HORIZONTAL : &'static str = "─";
const TOP_LEFT_CORNER : &'static str = "┌";
const TOP_RIGHT_CORNER : &'static str = "┐";
const BOTTOM_LEFT_CORNER : &'static str = "└";
const BOTTOM_RIGHT_CORNER : &'static str = "┘";
const VERTICAL : &'static str = "│";
const CROSS : &'static str = "┼";
const TEE_RIGHT : &'static str = "├";
const TEE_LEFT : &'static str = "┤";
const TEE_DOWN : &'static str = "┬";
const TEE_UP : &'static str = "┴";

const TRANSPARENT_ROOK : &'static str = "♖";
const TRANSPARENT_KNIGHT : &'static str = "♘";
const TRANSPARENT_BISHOP : &'static str = "♗";
const TRANSPARENT_QUEEN : &'static str = "♕";
const TRANSPARENT_KING : &'static str = "♔";
const TRANSPARENT_PAWN : &'static str = "♙";

```

They may be useful again someday.

## 2143

I got things cleaned up and merged, I think the board is basically done for now. It's pointed out a need to extract some
of the UI to configuration, so I can change it more easily. I dislike the unicode pieces a lot, I imagine it's just the
font I'm using not being particular legible, but I think I want to aim in a direction of sprites rendered using a
terminal image api. This is a little bit trickier, and think my time is better spent elsewhere right now. When I do
that, I'll spend some time looking at custom font patching as well, as keeping it all text is pretty nice, and I don't
think it'd be too difficult to make some better looking pieces.

I think next I want to set up an integration test, the goal will be to evaluate some deep `perft`, and send the games
off to be 'validated' by `stockfish`, ideally in parallel (on both sides).

The idea would require building some better abstractions around `Engine`s, and would also work towards finishing the
movegen component. I do want to build the `Alteration` based model; but I think I'm close enough with my initial, naive
approach that it's worth bringing to completion if I can before starting on the next version.

I do want to get the UI finished soon, but it needs some design review, as the current deeply nested approach is a
little tricky without some distinct 'main loop'. This should be straightforward to build with `tokio`, but I need to
take some time to do it.

I've been working with `worktrees` recently; and I'm debating just having three branches running so I can switch between
subtasks. It'll make the `LOG` a little weird to maintain though -- parallel universes. Maybe I'll start adding the
branch to the entry? It should be a straightforward merge process.

I suppose I'll know by the next entry.

## 2201 - movegen-v2

Hi from a worktree. This worktree will track work on the 'movegen-v2', which is a new approach to tracking gamestate and
generating moves. I expect this branch to merge once over it's lifespan. I'm running a few of these worktrees
simultaneously, experimenting with the model, we'll see if it works.


# 8-OCT-2024

## 1827 - movegen-v2

Got some initial work done getting a better game-tracking structure in place (`HalfPly`) and tying it to the existing
`Move` struct. I also was able to abstract away a lot of the board rep stuff behind some interfaces.

Remaining is to organize the various `move` related code and start tearing out the old crappy stuff and building
something better. I briefly took a look in `notation-spike` at doing some const-time notation stuff, but I'm stymied by
the half-implementation there.

I've got plenty to do on this side now that the HalfPly work is done, I can start to build up the `Line` struct and
build up to something that'll replace the current `Game` struct.

Once that's done, I can start work on a movegen/legality checker. I think I want to build this as a collection of
small services, one to generate all moves (no legality checking), a second to check for legality independently, and
later evaluation services that can provide various evaluations, these evaluation tools can be tagged and run
independently to compare them to each other in situ. The main process will spawn a process which has the engine backend
and the some frontend that tracks boardstate, options-per-engine, etc. The main process can then spawn whatever child
processes it likes based on the request from the Engine.

It makes sense in my head, which means it almost certainly won't make sense anywhee else, but that's how it always goes.

# 10-OCT-2024

## 0017 - movegen-v2

Thinking about the `Notation` idea again. I think there is a different way I can approach it that won't encounter the
type problems I was having. In particular I can use an associated type to track the format of the notation, so that I
can convert internally between the formats.

Another option is just to have an unassociated family of types with mutual `From` implementations. One called `Index`,
one called `UCI`, and so on, then I can have them all be const impls, and everything in the crate can just use `Index`,
but with a const `into()` method which converts to the correct type at compile time?

I truly have no idea if that'll work, but it makes half sense in my head, and my lack of types for these things is
starting to hurt.

## 0100 - movegen-v2

I think I have a better plan now.

I'm going to reorganize the repo like this:


```

src/
    board/
        bit/         # Implementation of a bitboard-based representation. Not the bitboard type itself.
        piece/       # Implementation of a simpler array-of-objects representation.
        planned/     # ... there may be other representations I put here eventually as well.
        mod.rs       # Interface definitions and module stuff, Alter and Query live here.
    notation/
        # Some kind of notation management object, ideally mostly const-time, as a QoL thing. Much
        # easier to have a consistent way to represent notation, instead of the mix of octal and text and coords.
        index/    # Index-based move notation
        uci/      # UCI Move notation
        san/      # SAN Move notation
        fen/      # FEN gamestate parsing and generation
        pgn/      # PGN game history parsing and generation
        mod.rs
    move/
        rep/
            # This contains various compact move representations, and probably also the `alteration` type and it's
            # friends.
        gen/
            # Async Move generator, legality checker, etc.
    game/
        # This tracks the gamestate, it's home to:
        halfply/    # Represents a single move/
        line/
        variation/
        mod.rs       # The Game Representation object itself.
        interface.rs # The `Chess` trait goes here
    evaluator/
        # evaluation here, someday
    brain/
        # Async Hazel Engine
    engine/
        # UCI interface
        uci/
        driver/
            stockfish.rs # Wrapper around stockfish, for integration testing
            hazel.rs     # Hazel Engine implementation wrapper (wrapping brain.
            mod.rs
        mod.rs
    ui/
        model/
        widgets/
        app.rs
        mod.rs
    types/
        # generic/crossfunctional types that are used everywhere.
        bitboard/  # bitboard type implementation
        pextboard/ # pextboard type implementation
        mask/      # async process wrapper.
    constants/
        # Constants used throughout the program
```

It's a bit easier to see the structure of the thing that way. The 'brain' is the main process that the UI will spawn
and interact with. It will spawn the other components and maintain the engine state. I'd like it to have a little
scripting language/VM sort of thing to control it via the UI in richer ways than UCI would allow. Ultimately, like
everything, I want to make it's parts hotloadable so I can leave it running all the time and just reload the
subcomponents on the fly. Everything must scale to infinite machines, uptime must grow.

I'm not going to do this till I finish the new movegen stuff, which should get all the raw materials above (mod the
evaluator, movegen, and brain bits, I suppose) in place, and then I can start to reorganize and clean up some of the
duplication I have now.


# 12-OCT-2024

## 1500 - movegen-v2

I think I have to commit to the reorg now, but some things have changed which makes the above not _quite_ right. I
started to change it and I think I'm just going to get in and start moving things around on a clean commit of this
branch. After that I think I'm going to merge and go back to working on main. It turns out I'm not so great at sticking
to a single topic on a side project so branching isn't really what I need to do here, at least not till I've cleaned up
some of the tech mess I've left for myself.

Next update will hopefully have all that done and I can document it here.

Also on the list is going to be getting `mkdocs` set up for a wiki, and maybe a local docserver, if only to motivate me
to document more.

# 13-OCT-2024

## 2322 - movegen-v2

A great culling has occurred as I move things towards the unified `Square` approach to notation. I've more or less got
everything lined up, and am just chasing out bugs from the various bits that I tweaked incorrectly to return to
compiling. At time of writing, six failing tests remain.

The approach I took seems alright, though it was tricky in spots. Essentially I have everything rely on an implementor
of `SquareNotation`, which requires the implementor be capable of converting to a `Square`, which is a simple newtype
around `usize` that constrains to `0..64` and provides some convenient const-time functions for working with it. The
trait wraps and provides those functions for general use, eventually it would be nice to push as much to const-time
evaluation as possible.

This should make it easy to have alternative implementations I can experiment with if I find better represnetaitons
later, and should smooth the cutover should the need arise.

After getting `Square` setup throughout, I plan to do the same thing with `Move`, building my current, compact `Move`
representation into the 'Default' `MoveNotation` implementation, similar to `SquareNotation` and `Square`. Also `FEN`
needs to be factoryed in throughout. Here it will be much more likely that thr trait will be valuable, as there are lots
of different move representation schemes, so by implementing the trait, I can set up a canonical pipeline to convert
between different representations. Something like:

```rust

pub fn convert<M, N>(m: M) -> N where M: MoveNotation, N: MoveNotation {
    let i = Move::from<m>;
    N::from(i)
}

```

I don't think it's possible to write the more general:

```rust

impl<M, N> From<N> for M where M: MoveNotation, N: MoveNotation {
    fn from(n: N) -> M {
        let i = Move::from(n);
        M::from(i)
    }
}

```

because of orphan instances, but I think the first is good enough for now. I can always add more implementations later,
and within hazel, it's most likely that I'll want to move to `Move` anyway.

Lots of gruntwork to do, but I think the result will be worth it.

# 16-OCT-2024

## 2104

I merged it. I got everything working again and I merged it. I was able to delete a ton of old code that wasn't needed,
and really improve the ergonomics of working in `hazel`. Now that it's at least partially done, let me take stock of
where things live and what it cost.


### Costs:

I binned the `NOTATION_TO_INDEX` and related constant lookup tables, and replaced them with a common `Square` type that
is tucked behind a `SquareNotation` trait. This leaves a door open to change this later, but for the moment having a
consistent type for referring to squares everywhere, and allowing me to easily convert between rank/file/index is very
handy. Saves me writing octal constants everywhere, too.

I also deleted, more impactffully, the `Game` and Ply` types, these were trying to do way too much, and I have a better
approach in mind. This does mean I gave up my progress towards `perft`, but I think I can bet back there in a more
stable configuration and not have to worry about carrying the mess with me.

Two weeks of work, and I'm technically behind where I was, but I think it was worth it.

Oh, and the benchmarks are toast. I changed a lot of the bitboard API and that's all that really had benchmarks. I'm
going to get the thing fully working and then go back and write benches. Originally I had started this as a reason to
learn `criterion`, but rapidly I found I was less interested in chasing performance then in the core problem of a chess
engine itself, so it bitrot.

### Benefits and new Design

The main benefit is that this enables moving forward with a new design. The organization is slightly improved. It looks
like this now.


```

src/
    board/
        # Board Representation
        interface/
        <board reps>
    brain/
        # Primary Subsystem, manages all the other processes.
    constants/
        # Useful _constants_
    coup/
        # Move related code
        rep/
            # Move representation
        gen/
            # Move Generation
    engine/
        # UCI Interface Substystem
    evaluator/
        # Position Evaluation Subsystem
    game/
        # Game State Representation
    notation/
        # Notation Representation + Conversion to internal canonical formats.
        fen/
        uci/
        square/
        pgn/
        ...
    types/
        # General types that don't fit in other subsystems.
        bitboard/
        pextboard/
        color.rs
        direction.rs
        occupant.rs
        piece.rs
    ui/
        # UI code
        model/
            # View Models
        widgets/
            # Widgets
        app.rs
    util/
        # Utility functions and some types
```

Ultimately, the `hazel` lib will provide a single tokio process `hazel` which spawns a ``brain` process, which in turn
spawns whatever other subsystems it wants according to it's configuration. This will include a gamestate represnetaiton,
some way to speak UCI, evaluation and movegen, etc. The idea is that `brain` can dynamically adjust how it's set up,
connect with other hazel instances, etc.

The `hazel` process can also optionally spawn a UI, which will allow deeper insight into the engine state.

Already existing is an ability to wrap other engines, so another subproject here is to build some cross-testing tools as
well.

So the thing looks roughly like:

```

Hazel
|               /--A- Engine Driver Backend
|               |
|------ Brain --|--A- Move Generator(s)
|               |
|               |---- Game State
|               |
|               \--A- Evaluator(s)
|
|
|
\------ UI

```

The `Hazel` process will be the main entrypoint, and will have a VM+scripting language to control it and the other
processes. This might be custom, for fun, not sure. I might just embed lua or something, like a responsible adult.

The goal would be to have a DSL that can load games, do analysis, process that analysis, etc. Less an engine to beat
other engines, more an engine to study positions efficiently, etc.

The next step is to rebuild the Game Representation, it's mostly done, but I want to get Move and PGN represented in the
notation module and Move's abstracted towards the `MoveNotation` trait similar to what I did for`SquareNotation`, and
`Square`. This is already underway as well.

After that, MoveGen and Perft. I still have the old `Ply` code so I can borrow from it where it makes sense. This is new
territory, I want to build it as a `tokio` process that speaks over some API. Ultimately I want `hazel` to be natively
multi-system, so it can run it's components across any system you like, so building up the engine as a bunch of
independent components is going to be necessary. I'll probably take that opportunity to scaffold in the whole process,
and also get a diagnostic environment set up.

Somewhere in there I also want to make more progress on the UI.

My plan is to have two worktrees, `UI` and whatever thing I''m working on at the time. As I implement stuff, I can keep
chipping at the UI as I go.

