module TestAccount::Mul {
    public fun mul(x: u128, y: u128): u128 {
        let mult = x * y;
        spec {
            assert mult == x*y;
        };

        mult
    }

    #[test]
    fun mul_test() {
        assert!(mul(0, 0) == 0, 0);
        assert!(mul(1, 0) == 0, 0);
        assert!(mul(0, 2) == 0, 0);
        assert!(mul(1, 3) == 3, 0);
        assert!(mul(4, 1) == 4, 0);
        assert!(mul(4, 1) == 4, 0);
        assert!(mul(5, 2) == 10, 0);
        assert!(mul(2, 6) == 12, 0);
        assert!(mul(7, 7) == 49, 0);
    }

    #[test, expected_failure(arithmetic_error, location = TestAccount::Mul)]
    fun mul_overflow_failure_test() {
        let a = 1 << 64;
        let b = 1 << 64;
        assert!(mul(a, b) != 0, 0);
    }
}
