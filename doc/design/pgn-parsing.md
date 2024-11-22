# 22-NOV-2024

## 1400 - pgn

Added some notes from last night/this morning. I'm working towards the tokenized parser now, since I should be able to
directly load the variation from the tokenstream, it should be quite a bit simpler than my current weird treeparsing
scheme.

# 21-NOV-2024

## 2300 (or thereabouts) -- pgn

This was a note I took while trying to think of a better way to parse the pgn files into Variations.

----

Maybe rework the parser a bit?

parse instead into tokens as a first pass, then a normal recursive descent to turn that into a Variation.

Tokens are 

TagPair
SAN
PlyNumber
    Mainline == X. SAN
    WhiteVariationOpen == (X. SAN



TT WW BB TT WWW BB TT WWW BBB TT WW BBB BVOp  BBB TT WW BB BVOp  BB TT WWW BB TT WWWC TT WWW BBC TT WWW BB TT WWWW WVO WWW BBBB TT WWWWC TBla BBBB TT WW BB XXX
1. d4 d5 2. Bf4 c6 3. Nf3 Nf6 4. e3 Nh5 (4... Bf5 5. c4 a5 (5... e6 6. Qb3 b6 7. Nc3) 6. Nc3 e6) 5. Be5 f6 6. Bxb8 (6. Bg3 Nxg3 7. hxg3) 6... Rxb8 7. c4 g6 0-1

Unique Tokens:

TT  == Turn number
TBla == Half-turn number (starts on black's turn
WW  == White Move
BB  == Black Move
BVOp == Black Variation Open
C == Close Variation
WVOp == White Variation Open
XXX == Halt condition

Translates to:

v = Variation

v.setup(START_POSITION_FEN)     // Initialization
                                // TT 1.
 .make(SAN::from("d4"))         // WW
 .make(SAN::from("d5"))         // BB
                                // TT 2.
 .make(SAN::from("Bf4"))        // WW
 .make(SAN::from("c6"))         // BB
                                // TT 3.
 .make(SAN::from("Nf3"))        // WW
 .make(SAN::from("Nf6"))        // BB
                                // TT 4.
 .make(SAN::from("e3"))         // WW
 .make(SAN::from("Nh5"))        // BB
 .variation_open()              // BVOp (4...  // Note this is just a hard open, if we don't close it correctly we will have a malformed file.
    .make(SAN::from("Bf5"))     // BB
                                // TT 5.
    .make(SAN::from("c4"))      // WW
    .make(SAN::from("a5"))      // BB
    .variation_open()           // BVOp (5...  // Now we've dropped two, so we must have two closes as well.
        .make(SAN::from("e6"))  // BB
                                // TT 6.
        .make(SAN::from("Qb3")) // WW
        .make(SAN::from("b6"))  // BB
                                // TT 7.
        .make(SAN::from("Nc3")) // WW
    .variation_close()          // C )
    .make(SAN::from("Nc3")      // WW
    .make(SAN::from("e6")       // BB
 .variation_close()             // C
                                // TT 5.
 .make(SAN::from("Be5"))        // WW
 .make(SAN::from("f6"))         // BB
                                // TT 6.
 .make(SAN::from("Bxb8"))       // WW
 .variation_open()              // WVO (6.
.make(SAN::from("Bg3"))     // WW
    .make(SAN::from("Nxg3"))    // BB
                                // TT 7.
    .make(SAN::from("hxg3"))    // WW
 .variation_close()             // C )
                                // TBla 6...
 .make(SAN::from("Rxb8"))       // BB
                                // TT 7.
 .make(SAN::from("c4"))         // WW
 .make(SAN::from("g6"))         // BB
 .commit_all()                  // XXX 0-1

This implies some semantics about this format:

1. Variations are sigil delimited.
2. The variation always starts on the turn of previously made `move`, that is, side-to-move does not change on the first make after opening a variation
3. The variation always returns play to the player who would have played had we not started the variation.
4. We will need to be able to see the current state of the tree at each `make` invocation, possibly rewinding/unwinding after a variation is closed.

I didn't do any finer-grained commit/transaction stuff, but I suspect that could be used to better effect (maybe when parallel processing later?)

I think to achieve #4 I'll have to calculate context 'from scratch' each time, using a cursor to navigate to the position written into the log.

The parser could take direct control and write into the log without constraint, so long as the parser is correct, the resulting log should be navigable
to create any necessary gamestate. That can then be optimized with caching or whatever.









     
     .make(SAN::from("
     .make(SAN::from("
     .make(SAN::from("
     .make(SAN::from("



