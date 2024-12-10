# 9-DEC-2024

## 1800 (or thereabouts) - familiars

(LGN
    (Headers
        (Tag Key Value)
        (Tag Key Value)
        ...
    )
    (Notation
        (Setup FEN)
        (D2 D4 'P) (D7 D5 'P)
        (C1 F4 'B) (G8 F6 'N)
        (Var nil (B1 C3 'N))
        (Var nil (E7 E6 'P))
        (E2 E3 'P) (Var (G1 F3 'N) (E7 E6 'P))
                   (E7 E6 'P)
    )
)

Is equivalent to:

[Header Stuff]

1. d4 d5
2. Bf4 Nf6 (2... Nc3) (2... e6)
3. e3 (3. Nf3 e6)
3... e6

--

The benefits of LGN ("Lispy Game Notation")

The mainline can be found by just filtering out all the Var blocks.
Unambiguous move notation, no need to understand the rules of chess, move included with every piece
Natural to extend with `(inline /path/to/file)`, which will make database representation a little nicer I think.

Open Questions

Maybe piece elision is fine where it's unambiguous, never for captures, but regular moves for sure.



