
Scripting language example:


load TYPE "path/to/fen/or/pgn/or/other/type" -> Loads to 'Active Game' ("AG") Register
store ARGS -> Stores the 'Active Game' in the game database
analyze [ARGS] -> no args = analyze the game in the AG register, with args, select a game or list of games to analyze from the DB.

control flow is assembly-like, the goal is for this to be a bytecode


Game should just be a game rep, it just keeps track of a game, the engine is agnostic to the gamestate, and gets the
full line in the `position` move, so it can just be a bare representation.

Game can then support #<< and #[], #[] gives a mutable reference to that move of the game, << will add a variation, =
will change the mainline, the object it gives will have a .variations() method which gives back the list of variation
moves or an empty list.


let mut g = Game::default();

g << UCI::from("d2d4") // make a move at the end of the current mainline
FEN::from(g) // FEN of the game after 1. d4
let m0 = g[0] // mutable reference to the 0th halfply of the game
m0 << UCI::from("e2e4") // a variation at the beginning of the game
FEN::from(g) // FEN of the game after 1. d4 still.
g[0].variations() // the Line containing only the moves up to and including this variation.
g << UCI::from("d7d5") // play 1. .. d5 , the mainline is now 1. d4 d5, and there is a 0th-move variation of 1. e4
g[1] << UCI::from("e7e5") // play 1. .. e5, the mainline is still 1. d4 d5, and there is a 1st-move variation of 1. d4 e5, and a 0th move variation of 1. e4

and so on. None of this updates a board state, no legality checking, just record the moves. A Game should also be able
to list all notated variations into a big (line, vec<line>) tuple, where the selected line is the mainline and the
others are all other lines.


MoveNotation has two targets, move-as-halfply, and move-as-Move. The former is for this game tracking thing, the latter
for the engine.



