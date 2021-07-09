use super::*;
use hazel::wizard::*;


bench!(
    group: Wizard,
    pretty: "allocation/empty",
    name: new,
    test: { Wizard::new() },
    where:
);

bench!(
    group: Wizard,
    pretty: "allocation/new",
    name: new,
    test: { Wizard::new() },
    where:
);

bench!(
    group: Wizard,
    pretty: "initialize_piece/1 - Rook initialization",
    name: initialize_piece,
    test: { w.initialize_piece(Piece::Rook) },
    where:
        w => Wizard::new();
);

bench!(
    group: Wizard,
    pretty: "initialize_piece/1 - Bishop initialization",
    name: initialize_piece,
    test: { w.initialize_piece(Piece::Bishop) },
    where:
        w => Wizard::new();
);