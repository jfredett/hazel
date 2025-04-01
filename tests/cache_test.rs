use hazel_core::ben::BEN;
use hazel_representation::game::position::Position;
use hazel_util::cache::Cache;

#[test]
fn cache_test() {
    let cache = Cache::new();

    let p = Position::new(BEN::start_position());

    assert_eq!(cache.size(), 0);
    cache.get(p.zobrist().position);
    assert_eq!(cache.size(), 0);
    cache.set(p.zobrist().position, p);
    assert_eq!(cache.size(), 1);
}
