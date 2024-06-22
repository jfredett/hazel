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
