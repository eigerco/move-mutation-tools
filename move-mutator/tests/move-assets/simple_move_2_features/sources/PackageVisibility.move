module TestAccount::PackageVisibility {
    package fun add_internal(a: u64, b: u64): u64 {
        a + b
    }

    package fun max_internal(a: u64, b: u64): u64 {
        if (a > b) { a } else { b }
    }

    #[test]
    fun test_add_internal() {
        assert!(add_internal(5, 10) == 15);
        assert!(add_internal(0, 0) == 0);
        assert!(add_internal(100, 200) == 300);
    }

    #[test]
    fun test_max_internal() {
        assert!(max_internal(5, 10) == 10);
        assert!(max_internal(10, 5) == 10);
        assert!(max_internal(7, 7) == 7);
    }
}
