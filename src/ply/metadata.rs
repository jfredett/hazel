use super::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    // TODO: Pack this structure, it should fit in a u32 comfortably.

    // The File where the en passant square appears
    //bits="0:2", ty="enum"
    en_passant_file: File,
    //packed_field(bits="3")]
    has_en_passant: bool,
    //packed_field(bits="4", ty="enum")]
    pub to_move: Color,
    //packed_field(bits="5")]
    pub white_castle_short: bool,
    //packed_field(bits="6")]
    pub white_castle_long: bool,
    //packed_field(bits="7")]
    pub black_castle_short: bool,
    //packed_field(bits="8")]
    pub black_castle_long: bool,
    //packed_field(bits="9:15")]
    pub half_move_clock: u8,
    //packed_field(bits="16:31", size_bits="15", endian="msb")]
    pub full_move_clock: u16
}


impl Metadata {
    pub fn en_passant(self) -> Option<Bitboard> {
        if self.has_en_passant {
            let ep_rank = if self.to_move.is_black() { *RANK_3 } else { *RANK_6 };
            Some(ep_rank & self.en_passant_file.to_bitboard())
        } else {
            None
        }
    }
    
    pub fn half_move_tick(&mut self) { self.half_move_clock += 1; }
    pub fn half_move_untick(&mut self) { self.half_move_clock -= 1; }
    pub fn half_move_reset(&mut self) { self.half_move_clock = 0; }
    
    pub fn full_move_tick(&mut self) { 
        if self.to_move.is_black() { self.full_move_clock += 1; }
        self.to_move = !self.to_move;
    }
    pub fn full_move_untick(&mut self) { 
        if self.to_move.is_white() { self.full_move_clock -= 1; }
        self.to_move = !self.to_move;
    }
    pub fn full_move_reset(&mut self) { self.full_move_clock = 0; }

    pub fn can_castle_short(self, color: Color) -> bool {
        match color {
            Color::WHITE => self.white_castle_short,
            Color::BLACK => self.black_castle_short
        }
    }

    pub fn can_castle_long(self, color: Color) -> bool {
        match color {
            Color::WHITE => self.white_castle_long,
            Color::BLACK => self.black_castle_long
        }
    }
    
    pub fn set_en_passant(&mut self, value: Option<usize>) {
        match value {
            Some(idx) => { 
                self.has_en_passant = true; 
                self.en_passant_file = FILES[idx % 8];
            },
            None => self.has_en_passant = false
        }
        
    }
}

impl Default for Metadata {
    fn default() -> Self {
        // metadata for initial position
        Metadata {
            // this is just a dummy value, since has_en_passant is false.
            en_passant_file: File::A,
            has_en_passant: false,
            to_move: Color::WHITE,
            white_castle_short: true,
            white_castle_long: true,
            black_castle_short: true,
            black_castle_long: true,
            half_move_clock: 0,
            full_move_clock: 1
        }
    }
}