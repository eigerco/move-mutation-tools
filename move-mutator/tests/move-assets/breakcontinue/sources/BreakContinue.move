module TestAccount::BreakContinue {
    fun sum_intermediate(n: u64): u64 {
        let sum = 0;
        let i = 0;
        loop {
            i = i + 1;
            if (i % 10 == 0) continue;
            if (i > n) break;
            sum = sum + i
        };

        sum
    }

    // We won't be able to kill one mutant:
    //  - if (n % i <= 0) break;
    #[test]
    fun sum_intermediate_test() {
        assert!(sum_intermediate(0) == 0, 0);
        assert!(sum_intermediate(1) == 1, 0);
        assert!(sum_intermediate(2) == 3, 0);
        assert!(sum_intermediate(3) == 6, 0);
        assert!(sum_intermediate(4) == 10, 0);
        assert!(sum_intermediate(5) == 15, 0);
        assert!(sum_intermediate(6) == 21, 0);
        assert!(sum_intermediate(7) == 28, 0);
        assert!(sum_intermediate(8) == 36, 0);
        assert!(sum_intermediate(9) == 45, 0);
        assert!(sum_intermediate(10) == 45, 0);
        assert!(sum_intermediate(11) == 56, 0);
    }
}
