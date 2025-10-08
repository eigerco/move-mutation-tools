module TestAccount::NewCastSyntax {
    fun u64_to_u128(x: u64): u128 {
        x as u128
    }

    fun add_and_cast(a: u64, b: u64): u128 {
        (a + b) as u128
    }

    fun multiply_and_widen(a: u64, b: u64): u128 {
        (a as u128) * (b as u128)
    }

    fun compare_after_cast(x: u64, threshold: u128): bool {
        (x as u128) > threshold
    }

    fun cast_down_if_small(x: u128): u64 {
        if (x <= 18446744073709551615) {
            x as u64
        } else {
            0
        }
    }

    fun divide_and_cast(numerator: u64, denominator: u64): u128 {
        (numerator / denominator) as u128
    }

    #[test]
    fun test_u64_to_u128() {
        assert!(u64_to_u128(0) == 0);
        assert!(u64_to_u128(100) == 100);
        assert!(u64_to_u128(999999) == 999999);
    }

    #[test]
    fun test_add_and_cast() {
        assert!(add_and_cast(10, 20) == 30);
        assert!(add_and_cast(0, 0) == 0);
        assert!(add_and_cast(1000, 2000) == 3000);
    }

    #[test]
    fun test_multiply_and_widen() {
        assert!(multiply_and_widen(10, 20) == 200);
        assert!(multiply_and_widen(100, 100) == 10000);
        assert!(multiply_and_widen(0, 999) == 0);
    }

    #[test]
    fun test_compare_after_cast() {
        assert!(compare_after_cast(100, 50) == true);
        assert!(compare_after_cast(50, 100) == false);
        assert!(compare_after_cast(100, 100) == false);
    }

    #[test]
    fun test_cast_down_if_small() {
        assert!(cast_down_if_small(100) == 100);
        assert!(cast_down_if_small(0) == 0);
        assert!(cast_down_if_small(1000) == 1000);
    }

    #[test]
    fun test_divide_and_cast() {
        assert!(divide_and_cast(100, 10) == 10);
        assert!(divide_and_cast(50, 2) == 25);
        assert!(divide_and_cast(7, 2) == 3);
    }
}
