use crate::{
    bitboard::Bitboard,
    constants::{
        Direction, 
        RANK_1, RANK_8,
        A_FILE, H_FILE,
        RANK_MASKS, FILE_MASKS,
        EDGES, CORNERS
    }
};

lazy_static! {
    /// A lookup table to convert a rook on an index -> it's unblocked attack squares, needed for magics
    pub static ref NOMINAL_ROOK_ATTACKS : [Bitboard; 64] = {
        let mut out = [Bitboard::empty(); 64];
        for rank in 0..8 {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let mut mask = !*EDGES;
                
                if rank == 0 { mask |= !*RANK_8; }
                if rank == 7 { mask |= !*RANK_1; }
                if file == 0 { mask |= !*H_FILE; }
                if file == 7 { mask |= !*A_FILE; }

                out[idx] = RANK_MASKS[rank] | FILE_MASKS[file];
                out[idx] &= mask;
                out[idx] &= !*CORNERS;
                out[idx] &= !Bitboard::from(1 << idx);
            }
        }
        return out
    };
    
    /// A lookup table to conver a bishop on an index -> it's unblocked attack squares, needed for magics
    pub static ref NOMINAL_BISHOP_ATTACKS : [Bitboard; 64] = {
        let mut out = [Bitboard::empty(); 64];
        for rank in 0..8 {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bishop = Bitboard::from(1 << idx);
                let mut attacks = bishop.clone();
                for d in [Direction::NW, Direction::NE, Direction::SW, Direction::SE] {
                    let mut bb = bishop.clone();
                    for _ in 0..8 {
                        bb |= bb.shift(d);
                    }
                    attacks |= bb;
                }
                out[idx] = attacks & !*EDGES & !bishop;
            }
        }
        return out
    };
}




    
    /*
    pub static ref ROOK_ATTACKS : [Magic; 64] = {
        // NOTE: This is unsafe because rust is _very_ weird about array initialization. There should be 
        // some way to work with an uninitialized array safely, and have the final block 'check' to make 
        // sure it's fully initialized at the end. This is a non-safe way of just doing that, it's unfortunate
        // you can't reference the index in the array initialization syntax.
        unsafe {
            let mut out: [MaybeUninit<Magic>; 64] = MaybeUninit::uninit().assume_init();
            let mut i = 0;
            for e in &mut out {
                *e = MaybeUninit::new(Magic::new_rook(i));
                i += 1;
            }
            
            mem::transmute::<_, [Magic; 64]>(out)
        }
    };
    
    pub static ref BISHOP_ATTACKS : [Magic; 64] = {
        // NOTE: This is unsafe because rust is _very_ weird... <snip see comment in ROOK_ATTACKS>
        unsafe {
            let mut out: [MaybeUninit<Magic>; 64] = MaybeUninit::uninit().assume_init();
            let mut i = 0;
            for e in &mut out {
                *e = MaybeUninit::new(Magic::new_bishop(i));
                i += 1;
            }
            
            mem::transmute::<_, [Magic; 64]>(out)
        }
    };
    */