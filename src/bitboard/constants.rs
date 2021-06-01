#[inline(always)]
#[allow(non_snake_case)]
pub fn NOTATION_TO_INDEX(notation: &str) -> usize {
    match notation {
        "a1" => { 0o00 }, "b1" => { 0o01 }, "c1" => { 0o02 }, "d1" => { 0o03 }, "e1" => { 0o04 }, "f1" => { 0o05 }, "g1" => { 0o06 }, "h1" => { 0o07 },
        "a2" => { 0o10 }, "b2" => { 0o11 }, "c2" => { 0o12 }, "d2" => { 0o13 }, "e2" => { 0o14 }, "f2" => { 0o15 }, "g2" => { 0o16 }, "h2" => { 0o17 },
        "a3" => { 0o20 }, "b3" => { 0o21 }, "c3" => { 0o22 }, "d3" => { 0o23 }, "e3" => { 0o24 }, "f3" => { 0o25 }, "g3" => { 0o26 }, "h3" => { 0o27 },
        "a4" => { 0o30 }, "b4" => { 0o31 }, "c4" => { 0o32 }, "d4" => { 0o33 }, "e4" => { 0o34 }, "f4" => { 0o35 }, "g4" => { 0o36 }, "h4" => { 0o37 },
        "a5" => { 0o40 }, "b5" => { 0o41 }, "c5" => { 0o42 }, "d5" => { 0o43 }, "e5" => { 0o44 }, "f5" => { 0o45 }, "g5" => { 0o46 }, "h5" => { 0o47 },
        "a6" => { 0o50 }, "b6" => { 0o51 }, "c6" => { 0o52 }, "d6" => { 0o53 }, "e6" => { 0o54 }, "f6" => { 0o55 }, "g6" => { 0o56 }, "h6" => { 0o57 },
        "a7" => { 0o60 }, "b7" => { 0o61 }, "c7" => { 0o62 }, "d7" => { 0o63 }, "e7" => { 0o64 }, "f7" => { 0o65 }, "g7" => { 0o66 }, "h7" => { 0o67 },
        "a8" => { 0o70 }, "b8" => { 0o71 }, "c8" => { 0o72 }, "d8" => { 0o73 }, "e8" => { 0o74 }, "f8" => { 0o75 }, "g8" => { 0o76 }, "h8" => { 0o77 },
        _ => { panic!("Unrecognized notation {}", notation) }
    }
}

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

    pub static ref COORDS_TO_INDEX: [[usize; 8]; 8] = {
        let mut m = [[0; 8]; 8];

        for idx in 0..64 {
            m[idx >> 3][idx % 8] = idx
        }

        m
    };

    pub static ref INDEX_TO_COORDS: [(usize, usize); 64] = {
        let mut m = [(0, 0); 64];

        let mut idx = 0;
        for (rank, rank_arr) in COORDS_TO_INDEX.iter().enumerate() {
            for file in rank_arr {
                m[idx] = (rank, *file % 8);
                idx += 1;

            }
        }
        m
    };
}


