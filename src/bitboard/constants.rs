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

        m
    };

    pub static ref COORDS_TO_INDEX: HashMap<(usize, usize), usize> = {
        let mut m = HashMap::new();

        for idx in 0..64 {
            m.insert((idx >> 3, idx % 8), idx);
        }

        m
    };

    pub static ref COORDS_TO_NOTATION: HashMap<(usize, usize), &'static str> = {
        let mut m = HashMap::new();

        for (coords, idx) in COORDS_TO_INDEX.iter() {
            m.insert(*coords, INDEX_TO_NOTATION[*idx]);
        }

        m
    };

    pub static ref INDEX_TO_COORDS: [(usize, usize); 64] = {
        let mut m = [(0, 0); 64];

        for (coords,idx) in COORDS_TO_INDEX.iter() {
            m[*idx] = *coords;
        }

        m
    };
}


