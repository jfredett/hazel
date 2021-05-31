use std::collections::HashMap;

lazy_static! {
    pub static ref INDEX_TO_NOTATION: [&'static str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
    ];

    pub static ref NOTATION_TO_INDEX: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();

        for (idx, notation) in INDEX_TO_NOTATION.iter().enumerate() {
            m.insert(*notation, idx);
        }

        //m.insert("a1", 0)  ; m.insert("b1", 1)  ; m.insert("c1", 2)  ; m.insert("d1", 3)  ; m.insert("e1", 4)  ; m.insert("f1", 5)  ; m.insert("g1", 6)  ; m.insert("h1", 7)  ;
        //m.insert("a2", 8)  ; m.insert("b2", 9)  ; m.insert("c2", 10) ; m.insert("d2", 11) ; m.insert("e2", 12) ; m.insert("f2", 13) ; m.insert("g2", 14) ; m.insert("h2", 15) ;
        //m.insert("a3", 16) ; m.insert("b3", 17) ; m.insert("c3", 18) ; m.insert("d3", 19) ; m.insert("e3", 20) ; m.insert("f3", 21) ; m.insert("g3", 22) ; m.insert("h3", 23) ;
        //m.insert("a4", 24) ; m.insert("b4", 25) ; m.insert("c4", 26) ; m.insert("d4", 27) ; m.insert("e4", 28) ; m.insert("f4", 29) ; m.insert("g4", 30) ; m.insert("h4", 31) ;
        //m.insert("a5", 32) ; m.insert("b5", 33) ; m.insert("c5", 34) ; m.insert("d5", 35) ; m.insert("e5", 36) ; m.insert("f5", 37) ; m.insert("g5", 38) ; m.insert("h5", 39) ;
        //m.insert("a6", 40) ; m.insert("b6", 41) ; m.insert("c6", 42) ; m.insert("d6", 43) ; m.insert("e6", 44) ; m.insert("f6", 45) ; m.insert("g6", 46) ; m.insert("h6", 47) ;
        //m.insert("a7", 48) ; m.insert("b7", 49) ; m.insert("c7", 50) ; m.insert("d7", 51) ; m.insert("e7", 52) ; m.insert("f7", 53) ; m.insert("g7", 54) ; m.insert("h7", 55) ;
        //m.insert("a8", 56) ; m.insert("b8", 57) ; m.insert("c7", 58) ; m.insert("d8", 59) ; m.insert("e8", 60) ; m.insert("f8", 61) ; m.insert("g8", 62) ; m.insert("h8", 63) ;

        m
    };

    pub static ref COORDS_TO_INDEX: HashMap<(usize, usize), usize> = {
        let mut m = HashMap::new();

        for idx in 0..64 {
            m.insert((idx >> 3, idx % 8), idx);
        }

        //m.insert((0,0), 0)  ; m.insert((1,0), 1)  ; m.insert((2,0), 2)  ; m.insert((3,0), 3)  ; m.insert((4,0), 4)  ; m.insert((5,0), 5)  ; m.insert((6,0), 6)  ; m.insert((7,0), 7)  ;
        //m.insert((0,1), 8)  ; m.insert((1,1), 9)  ; m.insert((2,1), 10) ; m.insert((3,1), 11) ; m.insert((4,1), 12) ; m.insert((5,1), 13) ; m.insert((6,1), 14) ; m.insert((7,1), 15) ;
        //m.insert((0,2), 16) ; m.insert((1,2), 17) ; m.insert((2,2), 18) ; m.insert((3,2), 19) ; m.insert((4,2), 20) ; m.insert((5,2), 21) ; m.insert((6,2), 22) ; m.insert((7,2), 23) ;
        //m.insert((0,3), 24) ; m.insert((1,3), 25) ; m.insert((2,3), 26) ; m.insert((3,3), 27) ; m.insert((4,3), 28) ; m.insert((5,3), 29) ; m.insert((6,3), 30) ; m.insert((7,3), 31) ;
        //m.insert((0,4), 32) ; m.insert((1,4), 33) ; m.insert((2,4), 34) ; m.insert((3,4), 35) ; m.insert((4,4), 36) ; m.insert((5,4), 37) ; m.insert((6,4), 38) ; m.insert((7,4), 39) ;
        //m.insert((0,5), 40) ; m.insert((1,5), 41) ; m.insert((2,5), 42) ; m.insert((3,5), 43) ; m.insert((4,5), 44) ; m.insert((5,5), 45) ; m.insert((6,5), 46) ; m.insert((7,5), 47) ;
        //m.insert((0,6), 48) ; m.insert((1,6), 49) ; m.insert((2,6), 50) ; m.insert((3,6), 51) ; m.insert((4,6), 52) ; m.insert((5,6), 53) ; m.insert((6,6), 54) ; m.insert((7,6), 55) ;
        //m.insert((0,7), 56) ; m.insert((1,7), 57) ; m.insert((2,7), 58) ; m.insert((3,7), 59) ; m.insert((4,7), 60) ; m.insert((5,7), 61) ; m.insert((6,7), 62) ; m.insert((7,7), 63) ;

        m
    };

    pub static ref COORDS_TO_NOTATION: HashMap<(usize, usize), &'static str> = {
        let mut m = HashMap::new();

        for (coords, idx) in COORDS_TO_INDEX.iter() {
            m.insert(*coords, INDEX_TO_NOTATION[*idx]);
        }

        //m.insert((0,0), "a1"); m.insert((1,0), "a2"); m.insert((2,0), "a3"); m.insert((3,0), "a4"); m.insert((4,0), "a5"); m.insert((5,0), "a6"); m.insert((6,0), "a7"); m.insert((7,0), "a8");
        //m.insert((0,1), "b1"); m.insert((1,1), "b2"); m.insert((2,1), "b3"); m.insert((3,1), "b4"); m.insert((4,1), "b5"); m.insert((5,1), "b6"); m.insert((6,1), "b7"); m.insert((7,1), "b8");
        //m.insert((0,2), "c1"); m.insert((1,2), "c2"); m.insert((2,2), "c3"); m.insert((3,2), "c4"); m.insert((4,2), "c5"); m.insert((5,2), "c6"); m.insert((6,2), "c7"); m.insert((7,2), "c8");
        //m.insert((0,3), "d1"); m.insert((1,3), "d2"); m.insert((2,3), "d3"); m.insert((3,3), "d4"); m.insert((4,3), "d5"); m.insert((5,3), "d6"); m.insert((6,3), "d7"); m.insert((7,3), "d8");
        //m.insert((0,4), "e1"); m.insert((1,4), "e2"); m.insert((2,4), "e3"); m.insert((3,4), "e4"); m.insert((4,4), "e5"); m.insert((5,4), "e6"); m.insert((6,4), "e7"); m.insert((7,4), "e8");
        //m.insert((0,5), "f1"); m.insert((1,5), "f2"); m.insert((2,5), "f3"); m.insert((3,5), "f4"); m.insert((4,5), "f5"); m.insert((5,5), "f6"); m.insert((6,5), "f7"); m.insert((7,5), "f8");
        //m.insert((0,6), "g1"); m.insert((1,6), "g2"); m.insert((2,6), "g3"); m.insert((3,6), "g4"); m.insert((4,6), "g5"); m.insert((5,6), "g6"); m.insert((6,6), "g7"); m.insert((7,6), "g8");
        //m.insert((0,7), "h1"); m.insert((1,7), "h2"); m.insert((2,7), "h3"); m.insert((3,7), "h4"); m.insert((4,7), "h5"); m.insert((5,7), "h6"); m.insert((6,7), "h7"); m.insert((7,7), "h8");

        m
    };

    pub static ref INDEX_TO_COORDS: [(usize, usize); 64] = {
        let mut m = [(0, 0); 64];

        for (coords,idx) in COORDS_TO_INDEX.iter() {
            m[*idx] = *coords;
        }

        dbg!(m);

        m

        //(0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
        //(0,1), (1,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1),
        //(0,2), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2),
        //(0,3), (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3),
        //(0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4),
        //(0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (7,5),
        //(0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6),
        //(0,7), (1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (7,7)
    };
}


