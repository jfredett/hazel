use hazel::notation::Square;
use hazel::types::Bitboard;
use hazel::types::Piece;
use hazel::types::pextboard;

/// This example is a Profile Hook for the PEXTBoard implementation. It simply executes random rook
/// attack calculations repeatedly to generate sampledata for perf.
const COUNT: usize = 1_000_000_000;
fn main() {
    let mut v = [Bitboard::empty(); 10];
    for i in 0..COUNT {
        let bb_in = rand::random::<u64>();
        let sq = Square::new(rand::random::<usize>() % 64);
        let blocks: Bitboard = Bitboard::from(bb_in);

        v[i % 10] = pextboard::attacks_for(Piece::Rook, sq, blocks);
    }
}
