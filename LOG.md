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
