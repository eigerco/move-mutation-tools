module TestAccount::StillSimple {
    fun sample1(x: u128, y: u128) {
        let _sum_r = x + y;

        // Impossible condition here:
        if ((x + y) < 0) abort 1;
    }

    // This test will generate mutants that will survive and indicate the impossible condition in the code.
    #[test]
    fun sample1_valid_usage_test() {
        sample1(0, 0);
        sample1(2, 1);
        sample1(1, 2);
    }

    inline fun apply(v: u64, predicate: |u64| bool): bool {
        predicate(v)
    }

    fun sample2(a1: u64, a2: u64): bool {
        let lamb = apply(0, |v| v != a1 + a2);
        lamb
    }

    #[test]
    fun sample2_valid_usage_test() {
        // The only case where it's gonna be false
        assert!(sample2(0, 0) == false, 0);

        assert!(sample2(0, 5) == true, 0);
        assert!(sample2(5, 0) == true, 0);

        assert!(sample2(1, 2) == true, 0);
        assert!(sample2(2, 1) == true, 0);
    }

    public fun sample3(n: u64, e: u64): u64 {
        if (e == 0) {
            1
        } else {
            n * sample3(n, e - 1)
        }
    }

    spec sample3 {
        pragma opaque;
    }

    #[test]
    fun sample3_valid_usage_test() {
        assert!(sample3(5, 0) == 1, 0);
        assert!(sample3(5, 1) == 5, 0);
        assert!(sample3(5, 2) == 25, 0);
        assert!(sample3(5, 3) == 125, 0);

        assert!(sample3(2, 8) == 256, 0);
        assert!(sample3(2, 16) == 65536, 0);

        assert!(sample3(1, 64) == 1, 0);
        assert!(sample3(1, 512) == 1, 0);
        assert!(sample3(0, 5) == 0, 0);

        // Huge number, let's assume it's not 5 instead :D
        assert!(sample3(521, 5) != 5, 0);
    }

    fun sample4(x: u128, y: u128) {
        loop {
            if (x > y) {
                y = y + 1;
                continue
            };
            if (y > x) {
                x = x + 1;
                continue
            };
            break
        };
    }

    #[test]
    fun sample4_valid_usage_test() {
        // Does this test even make any sense?
        sample4(0, 3);
        sample4(3, 0);
        sample4(5, 5);
    }

    fun sample5(x: u128, y: u128): u128 {
        (x - 1 as u128) + (y - 1 as u128)
    }

    #[test]
    fun sample5_valid_usage_test() {
        let expected_result = 4;
        let result = sample5(3, 3);
        assert!(expected_result == result, 1);

        let expected_result = 0;
        let result = sample5(1, 1);
        assert!(expected_result == result, 2);

        let expected_result = 8;
        let result = sample5(2, 8);
        assert!(expected_result == result, 3);
    }

    #[test, expected_failure(arithmetic_error, location = TestAccount::StillSimple)]
    fun sample5_substraction_overflow_failure_1_test() {
        let expected_result = 1;
        let result = sample5(0, 3);

        assert!(expected_result == result, 404);
    }

    #[test, expected_failure(arithmetic_error, location = TestAccount::StillSimple)]
    fun sample5_substraction_overflow_failure_2_test() {
        let expected_result = 4;
        let result = sample5(6, 0);

        assert!(expected_result == result, 404);
    }

    fun sample6(x: u128, y: u128): u128 {
        return (x + y - x*(y + 2)*x/y)
    }

    #[test, expected_failure]
    fun sample6_division_by_zero_failure_test() {
        assert!(sample6(50, 0) == 0, 0);
    }

    #[test]
    #[expected_failure]
    fun sample6_substraction_overflow_failure_test() {
        assert!(sample6(50, 5) == 0, 0);
    }

    #[test]
    fun sample6_valid_usage_test() {
        assert!(sample6(2, 4) == 0, 0);
        assert!(sample6(4, 14) == 0, 0);
        assert!(sample6(1, 2) == 1, 0);
        assert!(sample6(0, 2) == 2, 0);
        assert!(sample6(4, 16) == 2, 0);
    }
}
