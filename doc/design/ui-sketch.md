```
/--------------------------------------------------------------\
| Hazel (1500) v (3500) S  | /-------------------------------\ |
|    Bullet 1m+0           | | r | n | b | q | k | b | n | r | |
|    0:53 | 0:47           | |-------------------------------- |
|--------------------------| | p | p | p |   | p | p | p | p | |
| 1. d4 d5   | 1.21, 2.3   | |-------------------------------- |
| 2. Bf4 ... |             | |   |   |   |   |   |   |   |   | |
|            |    T        | |-------------------------------| |
|            |    i M      | |   |   |   | p |   |   |   |   | |
|     P      |    m o      | |-------------------------------| |
|     G      |    e v      | |   |   |   | P |   | B |   |   | |
|     N      |      e      | |-------------------------------| |
|            |    p        | |   |   |   |   |   |   |   |   | |
|            |    e        | |-------------------------------| |
|            |    r        | | P | P | P |   | P | P | P | P | |
|            |             | |-------------------------------| |
|            |             | | R | N |   | Q | K | B | N | R | |
|            |             | \-------------------------------/ |
|            |             |             Evaluation            |
|--------------------------------------------------------------|
| rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1     |
|--------------------------------------------------------------|
| Output from logger here, expands to fill assigned vertical   |
| space.                                                       |
|                                                              |
|                                                              |
|                                                              |
|                                                              |
|                                                              |
|--------------------------------------------------------------|
| $> input widget                                              |
\--------------------------------------------------------------/
```


Would be cool if the cells had shadow text with their names.

Ideally I can have a couple of these side by side, so I can 
compare between multiple, or for viewing simuls, etc.


This extends the output widget to fill available space, shows the current FEN for easy copying, the Top Left tag area
has a ticker-like scroll for the byline w/ the engine/player name and ELO, and shows time controls and game type

The board can eventually have Unicode pieces or even images if supported, and shows the eval of the current position.

This should easily fit 2 or 3 copies in a standard terminal for me, so ideally I can have a few 'slots' that
I can then swap out for different widgets as needed.

So a parent UI might be an arrangement of, as above, 32x64 windows into which I can shove whatever I like. I can enter a
'switcher' mode that switches between these tiles and executes commands on them. These tiles are borderless, so you'd
get something like (in a 64x128 window):


```
/--------------------------------------------------------------\ /--------------------------------------------------------------\
| Hazel (1500) v (3500) S  | /-------------------------------\ | | Hazel (1500) v (3500) S  | /-------------------------------\ |
|    Bullet 1m+0           | | r | n | b | q | k | b | n | r | | |    Bullet 1m+0           | | r | n | b | q | k | b | n | r | |
|    0:53 | 0:47           | |-------------------------------- | |    0:53 | 0:47           | |-------------------------------- |
|--------------------------| | p | p | p |   | p | p | p | p | | |--------------------------| | p | p | p |   | p | p | p | p | |
| 1. d4 d5   | 1.21, 2.3   | |-------------------------------- | | 1. d4 d5   | 1.21, 2.3   | |-------------------------------- |
| 2. Bf4 ... |             | |   |   |   |   |   |   |   |   | | | 2. Bf4 ... |             | |   |   |   |   |   |   |   |   | |
|            |    T        | |-------------------------------| | |            |    T        | |-------------------------------| |
|            |    i M      | |   |   |   | p |   |   |   |   | | |            |    i M      | |   |   |   | p |   |   |   |   | |
|     P      |    m o      | |-------------------------------| | |     P      |    m o      | |-------------------------------| |
|     G      |    e v      | |   |   |   | P |   | B |   |   | | |     G      |    e v      | |   |   |   | P |   | B |   |   | |
|     N      |      e      | |-------------------------------| | |     N      |      e      | |-------------------------------| |
|            |    p        | |   |   |   |   |   |   |   |   | | |            |    p        | |   |   |   |   |   |   |   |   | |
|            |    e        | |-------------------------------| | |            |    e        | |-------------------------------| |
|            |    r        | | P | P | P |   | P | P | P | P | | |            |    r        | | P | P | P |   | P | P | P | P | |
|            |             | |-------------------------------| | |            |             | |-------------------------------| |
|            |             | | R | N |   | Q | K | B | N | R | | |            |             | | R | N |   | Q | K | B | N | R | |
|            |             | \-------------------------------/ | |            |             | \-------------------------------/ |
|            |             |             Evaluation            | |            |             |             Evaluation            |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1     | | rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1     |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| Output from logger here, expands to fill assigned vertical   | | Output from logger here, expands to fill assigned vertical   |
| space.                                                       | | space.                                                       |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| $> input widget                                              | | $> input widget                                              |
\--------------------------------------------------------------/ \--------------------------------------------------------------/
/--------------------------------------------------------------\ /--------------------------------------------------------------\
| Hazel (1500) v (3500) S  | /-------------------------------\ | | Hazel (1500) v (3500) S  | /-------------------------------\ |
|    Bullet 1m+0           | | r | n | b | q | k | b | n | r | | |    Bullet 1m+0           | | r | n | b | q | k | b | n | r | |
|    0:53 | 0:47           | |-------------------------------- | |    0:53 | 0:47           | |-------------------------------- |
|--------------------------| | p | p | p |   | p | p | p | p | | |--------------------------| | p | p | p |   | p | p | p | p | |
| 1. d4 d5   | 1.21, 2.3   | |-------------------------------- | | 1. d4 d5   | 1.21, 2.3   | |-------------------------------- |
| 2. Bf4 ... |             | |   |   |   |   |   |   |   |   | | | 2. Bf4 ... |             | |   |   |   |   |   |   |   |   | |
|            |    T        | |-------------------------------| | |            |    T        | |-------------------------------| |
|            |    i M      | |   |   |   | p |   |   |   |   | | |            |    i M      | |   |   |   | p |   |   |   |   | |
|     P      |    m o      | |-------------------------------| | |     P      |    m o      | |-------------------------------| |
|     G      |    e v      | |   |   |   | P |   | B |   |   | | |     G      |    e v      | |   |   |   | P |   | B |   |   | |
|     N      |      e      | |-------------------------------| | |     N      |      e      | |-------------------------------| |
|            |    p        | |   |   |   |   |   |   |   |   | | |            |    p        | |   |   |   |   |   |   |   |   | |
|            |    e        | |-------------------------------| | |            |    e        | |-------------------------------| |
|            |    r        | | P | P | P |   | P | P | P | P | | |            |    r        | | P | P | P |   | P | P | P | P | |
|            |             | |-------------------------------| | |            |             | |-------------------------------| |
|            |             | | R | N |   | Q | K | B | N | R | | |            |             | | R | N |   | Q | K | B | N | R | |
|            |             | \-------------------------------/ | |            |             | \-------------------------------/ |
|            |             |             Evaluation            | |            |             |             Evaluation            |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1     | | rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1     |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| Output from logger here, expands to fill assigned vertical   | | Output from logger here, expands to fill assigned vertical   |
| space.                                                       | | space.                                                       |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|                                                              | |                                                              |
|--------------------------------------------------------------| |--------------------------------------------------------------|
| $> input widget                                              | | $> input widget                                              |
\--------------------------------------------------------------/ \--------------------------------------------------------------/
| : Command line                                                                                                   |Status Info |
```


The selected window would get a different color, ideally mouse support would happen.


Alternative tiles could be created in this strict size (I like 32x64 as a unit I think) which show different information
(maybe a summary of how all currently executing games are going, w/e

The borders will be 'real' borders, as Ratatui provides, but I think this gets the idea.

Keybinds:

<Esc> sends you to 'command mode', where the 'command line' becomes active, ideally it looks a little more vim-y, to
distinguish it from the input widgets. `hjkl` will select each window, `i` sends you to 'interactive' mode where can
enter in the input widget to send moves in PGN, UCI, or other format. Some metadata might be provided in place of the
`$>` to indicate which submode you're in/which command language you're 'speaking'

That's it, those are all the bindings, everything else is sent to the input widget and onto the engine.

Building this needs a layout, so I'll start there.

```
Layout:
    Root:
        Vertical
            Grid: (Should be infinite except for a reserved section, shows a WINDOWSIZE frame of complete tiles)
                Vec<Tile>: 32x64*n tiles
            CommandLine


Tile:
    Vertical:
        Upper Display
            Horizontal:
                Info
                    Vertical:
                        Info Line
                            Ticker(Entrant + ELO)
                                Horizontal
                                    White
                                    Black
                            Static(Time Control)
                            Static(Time Remaining)
                                Horizontal
                                    White
                                    Black
                        Game History
                            Horizontal
                                PGN
                                Time Used

                Board
        FEN
        Output - ScrollView (tui_scrollbar?)
        Input - tui_input
```

I think I want to look at this differently


There the following components (sizes exclude borders):


[Tile] 32x64
    Info Section: 17x27
        Title Card Block 3x27
            Entrant + ELO (Ticker) 1x27
            Time Control 1x27
            Time Remaining 1x27
        PGN Column 14x13
        Query Column 14x13 -- (Eventually this should be switchable to other kinds of info colums, Move time is shown)
    Board Section
        Board 27x34
        Query Line 1x34 -- (Evaluation shown here, but this should be an arbitrary query line that can be customized)
    Query Line 1x62
    Engine IO Section
        Output 7x62
        Input 1x58 (` $> ` is four characters)
Status Line 1xWIDTH
    Command Line 1xWIDTH-STATUSLENGTH
    Status Info STATUSLENGTH

the `[Tile]` section is a grid of tiles rendered on some canvas over which we have a 'camera' that can pan around.
Eventually supporting a 'zoom' level (see below) would be cool.

I'd like the various 'Query' sections above to be somewhat configurable, eventually pluggable. They'd contain queries
against the UI or engine, so it could support arbitrary custom messages for specific engines, etc. I can placeholder 
them for now with a generic `placeholder` widget.


Schematically:

```
INFO | BOARD
------------
QUERY TICKER
------------
    IO
```

The tricky thing is the mixed split, I think if I rearrange this as:

```
GAME
-----
QUERY
-----
IO
```

The layout will be easier


----




Eventually it'd be cool to use the smaller representation that's native to the `Ply` struct and current (27-SEP-2024)
`BoardWidget` impl to 'shrink' a game to an 8x8 size so more can be fit on the grid, selecting one would 'expand' it


```
____________________________________________________________
|RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR|
|PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|pppppppp||pppppppp||pppppppp||pppppppp||pppppppp||pppppppp|
|rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr|
------------------------------------------------------------
|RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR|
|PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|pppppppp||pppppppp||pppppppp||pppppppp||pppppppp||pppppppp|
|rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr|
------------------------------------------------------------
|RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR||RNBQKBNR|
|PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP||PPPPPPPP|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|........||........||........||........||........||........|
|pppppppp||pppppppp||pppppppp||pppppppp||pppppppp||pppppppp|
|rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr||rnbqkbnr|
------------------------------------------------------------
```

That more or less fits in the same size tile, but shows, y'know, a lot more games. Could likely see some improvement.
