// A familiar is a cursor on a ChessAction Log which maintains a gamestate and can be rolled
// forward/backward to different positions within the game. it will be responsible for talking to
// caches/doing other logic to make that process efficient, there will be many kinds of familiars,
// the most basic is the Representation Familiar, which takes some `Alter + Query` structure and
// does the default, pure-alter based approach to maintianing the gamestate. This is designed for
// maximum compatibility.
//
// All familiars implement the `Play` trait over the `ChessAction` type. 
