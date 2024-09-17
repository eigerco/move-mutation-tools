module TestAccount::Operators {
    fun sum(x: u64, y: u64): u64 {
        x + y
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
        x - y
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
        x * y
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
        x % y
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
        x / y
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
        x & y
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
        x | y
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
        x ^ y
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
        x << y
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
        x >> y
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

    fun logical_or(x: bool, y: bool): bool {
        x || y
    }

    #[test]
    fun logical_or_valid_usage_test() {
        assert!(logical_or(true, true) == true, 0);
        assert!(logical_or(false, false) == false, 0);
        assert!(logical_or(true, false) == true, 0);
        assert!(logical_or(false, true) == true, 0);
    }

    fun logical_and(x: bool, y: bool): bool {
        x && y
    }

    #[test]
    fun logical_and_valid_usage_test() {
        assert!(logical_and(true, true) == true, 0);
        assert!(logical_and(false, false) == false, 0);
        assert!(logical_and(true, false) == false, 0);
        assert!(logical_and(false, true) == false, 0);
    }

    fun logical_not(x: bool): bool {
        !x
    }

    #[test]
    fun logical_not_valid_usage_test() {
        assert!(logical_not(true) == false, 0);
        assert!(logical_not(false) == true, 0);
    }

    fun eq(x: u8, y: u8): bool {
        x == y
    }

    #[test]
    fun eq_valid_usage_test() {
        assert!(eq(1, 1) == true, 0);
        assert!(eq(0, 0) == true, 0);
        assert!(eq(63, 63) == true, 0);

        assert!(eq(1, 0) == false, 0);
        assert!(eq(2, 1) == false, 0);
        assert!(eq(6, 5) == false, 0);
        assert!(eq(4, 7) == false, 0);
    }

    fun neq(x: u8, y: u8): bool {
        x != y
    }

    #[test]
    fun neq_valid_usage_test() {
        assert!(neq(1, 1) == false, 0);
        assert!(neq(0, 0) == false, 0);
        assert!(neq(63, 63) == false, 0);

        assert!(neq(1, 0) == true, 0);
        assert!(neq(2, 1) == true, 0);
        assert!(neq(6, 5) == true, 0);
        assert!(neq(4, 7) == true, 0);
    }

    fun lt(x: u64, y: u64): bool {
        x < y
    }

    #[test]
    fun lt_valid_usage_test() {
        assert!(lt(1, 1) == false, 0);
        assert!(lt(0, 0) == false, 0);
        assert!(lt(63, 63) == false, 0);

        assert!(lt(1, 0) == false, 0);
        assert!(lt(2, 1) == false, 0);
        assert!(lt(6, 5) == false, 0);
        assert!(lt(4, 7) == true, 0);
    }

    fun lte(x: u64, y: u64): bool {
        x <= y
    }

    #[test]
    fun lte_valid_usage_test() {
        assert!(lte(1, 1) == true, 0);
        assert!(lte(0, 0) == true, 0);
        assert!(lte(63, 63) == true, 0);

        assert!(lte(1, 0) == false, 0);
        assert!(lte(2, 1) == false, 0);
        assert!(lte(6, 5) == false, 0);
        assert!(lte(4, 7) == true, 0);
    }

    fun gt(x: u64, y: u64): bool {
        x > y
    }

    #[test]
    fun gt_valid_usage_test() {
        assert!(gt(1, 1) == false, 0);
        assert!(gt(0, 0) == false, 0);
        assert!(gt(63, 63) == false, 0);

        assert!(gt(1, 0) == true, 0);
        assert!(gt(2, 1) == true, 0);
        assert!(gt(6, 5) == true, 0);
        assert!(gt(4, 7) == false, 0);
    }

    fun gte(x: u64, y: u64): bool {
        x >= y
    }

    #[test]
    fun gte_valid_usage_test() {
        assert!(gte(1, 1) == true, 0);
        assert!(gte(0, 0) == true, 0);
        assert!(gte(63, 63) == true, 0);

        assert!(gte(1, 0) == true, 0);
        assert!(gte(2, 1) == true, 0);
        assert!(gte(6, 5) == true, 0);
        assert!(gte(4, 7) == false, 0);
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

    spec logical_or {
        ensures result == x || y;
    }

    spec logical_not {
        ensures result == !x;
    }

    spec eq {
        ensures result == (x == y);
    }

    spec neq {
        ensures result == (x != y);
    }

    spec lt {
        ensures result == (x < y);
    }

    spec lte {
        ensures result == (x <= y);
    }

    spec gt {
        ensures result == (x > y);
    }

    spec gte {
        ensures result == (x >= y);
    }
}
