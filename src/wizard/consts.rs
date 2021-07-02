pub(super) const BOARD_SIZE        : usize = 64;
pub(super) const ROOK_TABLE_SIZE   : usize = 4096;
pub(super) const BISHOP_TABLE_SIZE : usize = 256;
pub(super) const TABLE_SIZE        : usize = BOARD_SIZE * (ROOK_TABLE_SIZE + BISHOP_TABLE_SIZE);
/// This is the maximum shift we'll allow into the table, it's equal to
/// `Floor(Log_2(TABLE_SIZE)) + 1` (since ranges are not inclusive there)
pub(super) const MAX_SHIFT         : u8    = 19;