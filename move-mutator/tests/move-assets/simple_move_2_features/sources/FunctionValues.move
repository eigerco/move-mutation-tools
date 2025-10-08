module TestAccount::FunctionValues {
    // Helper inline function to apply a lambda
    inline fun apply(f: |u64| u64, x: u64): u64 {
        f(x)
    }

    // Helper inline function for binary operations
    inline fun apply2(f: |u64, u64| u64, x: u64, y: u64): u64 {
        f(x, y)
    }

    // Helper inline function for comparison operations
    inline fun apply2_bool(f: |u64, u64| bool, x: u64, y: u64): bool {
        f(x, y)
    }

    // Helper inline function for comparison operations
    inline fun apply2_conditional(f: |bool, bool| bool, x: bool, y: bool): bool {
        f(x, y)
    }

    // Test addition operator mutation (mutates to -, *, /, %)
    fun add_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x + y, a, b)
    }

    // Test subtraction operator mutation (mutates to +, *, /, %)
    fun sub_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x - y, a, b)
    }

    // Test multiplication operator mutation (mutates to +, -, /, %)
    fun mul_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x * y, a, b)
    }

    // Test division operator mutation (mutates to +, -, *, %)
    fun div_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x / y, a, b)
    }

    // Test modulo operator mutation (mutates to +, -, *, /)
    fun mod_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x % y, a, b)
    }

    // Test bitwise AND operator mutation (mutates to |, ^)
    fun and_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x & y, a, b)
    }

    // Test bitwise OR operator mutation (mutates to &, ^)
    fun or_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x | y, a, b)
    }

    // Test bitwise XOR operator mutation (mutates to &, |)
    fun xor_with_lambda(a: u64, b: u64): u64 {
        apply2(|x, y| x ^ y, a, b)
    }

    // Test left shift operator mutation (mutates to >>)
    fun lsh_with_lambda(a: u64, b: u8): u64 {
        apply(|x| x << b, a)
    }

    // Test right shift operator mutation (mutates to <<)
    fun rsh_with_lambda(a: u64, b: u8): u64 {
        apply(|x| x >> b, a)
    }

    // Test greater than operator mutation (mutates to <, >=, <=, ==, !=)
    fun gt_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| x > y, a, b)) true else false
    }

    // Test greater than or equal operator mutation
    fun gte_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| x >= y, a, b)) true else false
    }

    // Test less than operator mutation
    fun lt_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| x < y, a, b)) true else false
    }

    // Test less than or equal operator mutation
    fun lte_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| x <= y, a, b)) true else false
    }

    // Test equality operator mutation (requires references)
    fun eq_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| &x == &y, a, b)) true else false
    }

    // Test inequality operator mutation (requires references)
    fun neq_with_lambda(a: u64, b: u64): bool {
        if (apply2_bool(|x, y| &x != &y, a, b)) true else false
    }

    // Test logical AND operator mutation
    fun logical_and_with_lambda(a: bool, b: bool): bool {
        if (apply2_conditional(|x, y| x && y, a, b)) true else false
    }

    // Test logical OR operator mutation
    fun logical_or_with_lambda(a: bool, b: bool): bool {
        if (apply2_conditional(|x, y| x || y, a, b)) true else false
    }

    // Tests killing all mutants
    #[test]
    fun test_add_with_lambda() {
        assert!(add_with_lambda(3, 5) == 8, 0);
        assert!(add_with_lambda(7, 11) == 18, 1);
        assert!(add_with_lambda(0, 9) == 9, 2);
    }

    #[test]
    fun test_sub_with_lambda() {
        assert!(sub_with_lambda(10, 3) == 7, 0);
        assert!(sub_with_lambda(20, 5) == 15, 1);
        assert!(sub_with_lambda(8, 8) == 0, 2);
    }

    #[test]
    fun test_mul_with_lambda() {
        assert!(mul_with_lambda(3, 7) == 21, 0);
        assert!(mul_with_lambda(5, 9) == 45, 1);
        assert!(mul_with_lambda(0, 100) == 0, 2);
        assert!(mul_with_lambda(1, 100) == 100, 3);
    }

    #[test]
    fun test_div_with_lambda() {
        assert!(div_with_lambda(20, 4) == 5, 0);
        assert!(div_with_lambda(15, 3) == 5, 1);
        assert!(div_with_lambda(7, 2) == 3, 2);
        assert!(div_with_lambda(0, 5) == 0, 3);
    }

    #[test]
    fun test_mod_with_lambda() {
        assert!(mod_with_lambda(17, 5) == 2, 0);
        assert!(mod_with_lambda(23, 7) == 2, 1);
        assert!(mod_with_lambda(10, 3) == 1, 2);
        assert!(mod_with_lambda(8, 4) == 0, 3);
    }

    #[test]
    fun test_and_with_lambda() {
        assert!(and_with_lambda(12, 10) == 8, 0); // 1100 & 1010 = 1000
        assert!(and_with_lambda(7, 3) == 3, 1);   // 0111 & 0011 = 0011
        assert!(and_with_lambda(15, 8) == 8, 2);  // 1111 & 1000 = 1000
        assert!(and_with_lambda(5, 9) == 1, 3);   // 0101 & 1001 = 0001
    }

    #[test]
    fun test_or_with_lambda() {
        assert!(or_with_lambda(12, 10) == 14, 0); // 1100 | 1010 = 1110
        assert!(or_with_lambda(5, 3) == 7, 1);    // 0101 | 0011 = 0111
        assert!(or_with_lambda(8, 4) == 12, 2);   // 1000 | 0100 = 1100
        assert!(or_with_lambda(1, 2) == 3, 3);    // 0001 | 0010 = 0011
    }

    #[test]
    fun test_xor_with_lambda() {
        assert!(xor_with_lambda(12, 10) == 6, 0); // 1100 ^ 1010 = 0110
        assert!(xor_with_lambda(7, 3) == 4, 1);   // 0111 ^ 0011 = 0100
        assert!(xor_with_lambda(15, 5) == 10, 2); // 1111 ^ 0101 = 1010
        assert!(xor_with_lambda(8, 8) == 0, 3);   // 1000 ^ 1000 = 0000
    }

    #[test]
    fun test_lsh_with_lambda() {
        assert!(lsh_with_lambda(3, 2) == 12, 0);  // 3 << 2 = 12
        assert!(lsh_with_lambda(5, 3) == 40, 1);  // 5 << 3 = 40
        assert!(lsh_with_lambda(7, 1) == 14, 2);  // 7 << 1 = 14
        assert!(lsh_with_lambda(1, 5) == 32, 3);  // 1 << 5 = 32
    }

    #[test]
    fun test_rsh_with_lambda() {
        assert!(rsh_with_lambda(24, 2) == 6, 0);  // 24 >> 2 = 6
        assert!(rsh_with_lambda(40, 3) == 5, 1);  // 40 >> 3 = 5
        assert!(rsh_with_lambda(15, 1) == 7, 2);  // 15 >> 1 = 7
        assert!(rsh_with_lambda(128, 5) == 4, 3); // 128 >> 5 = 4
    }

    #[test]
    fun test_gt_with_lambda() {
        assert!(gt_with_lambda(10, 5) == true, 0);
        assert!(gt_with_lambda(5, 10) == false, 1);
        assert!(gt_with_lambda(5, 5) == false, 2);
    }

    #[test]
    fun test_gte_with_lambda() {
        assert!(gte_with_lambda(10, 5) == true, 0);
        assert!(gte_with_lambda(5, 10) == false, 1);
        assert!(gte_with_lambda(5, 5) == true, 2);
    }

    #[test]
    fun test_lt_with_lambda() {
        assert!(lt_with_lambda(5, 10) == true, 0);
        assert!(lt_with_lambda(10, 5) == false, 1);
        assert!(lt_with_lambda(5, 5) == false, 2);
    }

    #[test]
    fun test_lte_with_lambda() {
        assert!(lte_with_lambda(5, 10) == true, 0);
        assert!(lte_with_lambda(10, 5) == false, 1);
        assert!(lte_with_lambda(5, 5) == true, 2);
    }

    #[test]
    fun test_eq_with_lambda() {
        assert!(eq_with_lambda(5, 5) == true, 0);
        assert!(eq_with_lambda(5, 10) == false, 1);
        assert!(eq_with_lambda(0, 0) == true, 2);
        assert!(eq_with_lambda(100, 99) == false, 3);
    }

    #[test]
    fun test_neq_with_lambda() {
        assert!(neq_with_lambda(5, 10) == true, 0);
        assert!(neq_with_lambda(5, 5) == false, 1);
        assert!(neq_with_lambda(0, 1) == true, 2);
        assert!(neq_with_lambda(100, 100) == false, 3);
    }

    #[test]
    fun test_logical_and_with_lambda() {
        assert!(logical_and_with_lambda(true, true) == true, 0);
        assert!(logical_and_with_lambda(true, false) == false, 1);
        assert!(logical_and_with_lambda(false, true) == false, 2);
        assert!(logical_and_with_lambda(false, false) == false, 3);
    }

    #[test]
    fun test_logical_or_with_lambda() {
        assert!(logical_or_with_lambda(true, true) == true, 0);
        assert!(logical_or_with_lambda(true, false) == true, 1);
        assert!(logical_or_with_lambda(false, true) == true, 2);
        assert!(logical_or_with_lambda(false, false) == false, 3);
    }
}

