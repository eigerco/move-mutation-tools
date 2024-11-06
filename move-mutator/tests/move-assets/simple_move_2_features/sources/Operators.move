module TestAccount::Operators {
    fun sum(x: u64, y: u64): u64 {
        let ret = x;
        ret += y;
        ret
    }

    #[test]
    fun sum_valid_usage_test() {
        assert!(sum(0, 0) == 0, 0);
        assert!(sum(1, 0) == 1, 0);
        assert!(sum(0, 1) == 1, 0);
        assert!(sum(5, 5) == 10, 0);
        assert!(sum(50, 10) == 60, 0);
    }

    fun sub(x: u64, y: u64): u64 {
        let ret = x;
        ret -= y;
        ret
    }

    #[test]
    fun sub_valid_usage_test() {
        assert!(sub(0, 0) == 0, 0);
        assert!(sub(1, 0) == 1, 0);
        assert!(sub(5, 5) == 0, 0);
        assert!(sub(50, 10) == 40, 0);
    }

    #[test, expected_failure]
    fun sub_invalid_usage_test() {
        assert!(sub(0, 5) == 0, 0);
    }

    fun mul(x: u64, y: u64): u64 {
        let ret = x;
        ret *= y;
        ret
    }

    #[test]
    fun mul_valid_usage_test() {
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

    #[test, expected_failure(arithmetic_error, location = TestAccount::Operators)]
    fun mul_overflow_failure_test() {
        let a = 1 << 64;
        let b = 1 << 64;
        assert!(mul(a, b) != 0, 0);
    }

    fun mod(x: u64, y: u64): u64 {
        let ret = x;
        ret %= y;
        ret
    }

    #[test]
    fun mod_valid_usage_test() {
        assert!(mod(0, 1) == 0, 0);
        assert!(mod(0, 2) == 0, 0);
        assert!(mod(2, 2) == 0, 0);
        assert!(mod(4, 4) == 0, 0);

        assert!(mod(0, 3) == 0, 0);
        assert!(mod(1, 3) == 1, 0);
        assert!(mod(2, 3) == 2, 0);
        assert!(mod(3, 3) == 0, 0);
        assert!(mod(4, 3) == 1, 0);
        assert!(mod(5, 3) == 2, 0);
        assert!(mod(6, 3) == 0, 0);

        assert!(mod(67, 65) == 2, 0);
        assert!(mod(40, 45) == 40, 0);
    }

    // TODO: This test is important as it covers division by zero. At the moment - move-mutation test cannot detect if this test is missing
    #[test, expected_failure(arithmetic_error, location = TestAccount::Operators)]
    fun mod_invalid_usage_test() {
        assert!(mod(5, 0) != 1000, 0);
    }

    fun div(x: u64, y: u64): u64 {
        let ret = x;
        ret /= y;
        ret
    }

    #[test]
    fun div_valid_usage_test() {
        assert!(div(0, 1) == 0, 0);
        assert!(div(0, 2) == 0, 0);
        assert!(div(1, 2) == 0, 0);
        assert!(div(2, 2) == 1, 0);

        assert!(div(200, 10) == 20, 0);
        assert!(div(205, 10) == 20, 0);
        assert!(div(210, 10) == 21, 0);

        assert!(div(67, 65) == 1, 0);
        assert!(div(40, 45) == 0, 0);
    }

    // TODO: This test is important as it covers division by zero. At the moment - move-mutation test cannot detect if this test is missing
    #[test, expected_failure(arithmetic_error, location = TestAccount::Operators)]
    fun div_invalid_usage_test() {
        assert!(div(50, 0) != 1000, 0);
    }

    fun and(x: u64, y: u64): u64 {
        let ret = x;
        ret &= y;
        ret
    }

    #[test]
    fun and_valid_usage_test() {
        assert!(and(0, 0) == 0, 0);
        assert!(and(1, 0) == 0, 0);
        assert!(and(0, 2) == 0, 0);
        assert!(and(1, 1) == 1, 0);
        assert!(and(3, 3) == 3, 0);
        assert!(and(3, 2) == 2, 0);
        assert!(and(3, 4) == 0, 0);

        assert!(and(8, 5) == 0, 0);
        assert!(and(8, 9) == 8, 0);
        assert!(and(8, 63) == 8, 0);
        assert!(and(65, 64) == 64, 0);
    }

    fun or(x: u64, y: u64): u64 {
        let ret = x;
        ret |= y;
        ret
    }

    #[test]
    fun or_valid_usage_test() {
        assert!(or(0, 0) == 0, 0);
        assert!(or(1, 0) == 1, 0);
        assert!(or(0, 2) == 2, 0);
        assert!(or(1, 1) == 1, 0);
        assert!(or(3, 3) == 3, 0);
        assert!(or(3, 2) == 3, 0);
        assert!(or(3, 4) == 7, 0);

        assert!(or(8, 5) == 13, 0);
        assert!(or(8, 9) == 9, 0);
        assert!(or(8, 63) == 63, 0);
        assert!(or(65, 64) == 65, 0);
        assert!(or(128, 0) == 128, 0);
    }

    fun xor(x: u64, y: u64): u64 {
        let ret = x;
        ret ^= y;
        ret
    }

    #[test]
    fun xor_valid_usage_test() {
        assert!(xor(0, 0) == 0, 0);
        assert!(xor(1, 1) == 0, 0);
        assert!(xor(1, 0) == 1, 0);
        assert!(xor(0, 2) == 2, 0);
        assert!(xor(1, 2) == 3, 0);
        assert!(xor(128, 0) == 128, 0);
    }

    fun lsh(x: u64, y: u8): u64 {
        let ret = x;
        ret <<= y;
        ret
    }

    #[test]
    fun lsh_valid_usage_test() {
        assert!(lsh(0, 0) == 0, 0);
        assert!(lsh(1, 0) == 1, 0);
        assert!(lsh(2, 0) == 2, 0);
        assert!(lsh(3, 0) == 3, 0);
        assert!(lsh(16, 0) == 16, 0);
        assert!(lsh(17, 0) == 17, 0);

        assert!(lsh(0, 1) == 0, 0);
        assert!(lsh(1, 1) == 2, 0);
        assert!(lsh(2, 1) == 4, 0);
        assert!(lsh(3, 1) == 6, 0);
        assert!(lsh(16, 1) == 32, 0);
        assert!(lsh(17, 1) == 34, 0);
        assert!(lsh(18, 2) == 72, 0);
    }

    fun rsh(x: u64, y: u8): u64 {
        let ret = x;
        ret >>= y;
        ret
    }

    #[test]
    fun rsh_valid_usage_test() {
        assert!(rsh(0, 0) == 0, 0);
        assert!(rsh(1, 0) == 1, 0);
        assert!(rsh(2, 0) == 2, 0);
        assert!(rsh(3, 0) == 3, 0);
        assert!(rsh(16, 0) == 16, 0);
        assert!(rsh(17, 0) == 17, 0);

        assert!(rsh(0, 1) == 0, 0);
        assert!(rsh(1, 1) == 0, 0);
        assert!(rsh(2, 1) == 1, 0);
        assert!(rsh(3, 1) == 1, 0);
        assert!(rsh(16, 1) == 8, 0);
        assert!(rsh(17, 1) == 8, 0);
        assert!(rsh(18, 2) == 4, 0);
    }

    spec sum {
        ensures result == x+y;
    }

    spec sub {
        ensures result == x-y;
    }

    spec mul {
        ensures result == x*y;
    }

    spec mod {
        aborts_if y == 0;
        ensures result == x%y;
    }

    spec div {
        aborts_if y == 0;
        ensures result == x/y;
    }

    spec and {
        ensures result == int2bv(x) & int2bv(y);
    }

    spec or {
        ensures result == int2bv(x) | int2bv(y);
    }

    spec xor {
        ensures result == int2bv(x) ^ int2bv(y);
    }

    spec lsh {
        aborts_if y >= 64;
        ensures result == x<<y;
    }

    spec rsh {
        aborts_if y >= 64;
        ensures result == x>>y;
    }
}
