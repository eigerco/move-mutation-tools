module CafeAccount::MulUse {
    use TestAccount::Mul::mul;

    fun mul_usage(x: u128, y: u128): u128 {
        let z = x + 1;
        let w = y + 2;
        mul(z, w)
    }

    #[test]
    fun mul_usage_test() {
        assert!(mul_usage(0, 0) == 2, 0);
        assert!(mul_usage(1, 0) == 4, 0);
        assert!(mul_usage(6, 5) == 49, 0);
    }

    #[test, expected_failure(arithmetic_error, location = TestAccount::Mul)]
    fun mul_usage_overflow_failure_test() {
        let a = 1 << 64;
        let b = 1 << 64;
        assert!(mul_usage(a, b) != 0, 0);
    }
}
