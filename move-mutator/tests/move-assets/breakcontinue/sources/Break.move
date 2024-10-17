module TestAccount::Break {
    fun smallest_factor(n: u64): u64 {
        // assuming the input is not 0 or 1
        let i = 2;
        while (i <= n) {
            if (n % i == 0) break;
            i = i + 1
        };

        i
    }

    #[test]
    fun smallest_factor_test() {
        // These two should fail - but let's keep imperfect code here
        assert!(smallest_factor(0) == 2, 0);
        assert!(smallest_factor(1) == 2, 0);

        assert!(smallest_factor(2) == 2, 0);
        assert!(smallest_factor(9) == 3, 0);
        assert!(smallest_factor(35) == 5, 0);
        assert!(smallest_factor(91) == 7, 0);
    }
}
