module TestAccount::PositionalStructs {
    use std::string::{String, utf8};

    struct Wrapper(u64) has drop;

    struct Pair(u64, u64) has drop;

    fun double(w: Wrapper): u64 {
        w.0 * 2
    }

    fun pair_sum(p: Pair): u64 {
        p.0 + p.1
    }

    #[test]
    fun test_unwrap_and_double() {
        assert!(double(Wrapper(5)) == 10);
        assert!(double(Wrapper(10)) == 20);
        assert!(double(Wrapper(0)) == 0);
    }

    #[test]
    fun test_pair_sum() {
        assert!(pair_sum(Pair(5, 10)) == 15);
        assert!(pair_sum(Pair(0, 0)) == 0);
        assert!(pair_sum(Pair(100, 200)) == 300);
    }
}
