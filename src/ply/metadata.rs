use super::*;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Metadata: u8 {
        const WHITE_CASTLE_LONG  = 0b00000001;
        const WHITE_CASTLE_SHORT = 0b00000010;
        const BLACK_CASTLE_LONG  = 0b00000100;
        const BLACK_CASTLE_SHORT = 0b00001000;
        const EN_PASSANT         = 0b00010000;
        const BLACK_TO_MOVE      = 0b00100000;
        const IN_CHECK           = 0b01000000;
        // NOTE: Maybe this should be used for 'seen this position before?' if 1, then we need to lookup in the 3-fold transpo table or w/e
        const UNUSED             = 0b10000000;
        // convenience flags
        const DEFAULT            = 0b00001111;
    }
}