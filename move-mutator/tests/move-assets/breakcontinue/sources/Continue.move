module TestAccount::Continue {
    fun sum_intermediate(n: u64): u64 {
        let sum = 0;
        let i = 0;
        while (i < n) {
            i = i + 1;
            if (i % 10 == 0) continue;
            sum = sum + i;
        };

        sum
    }

    // A single mutant will survive here, and it's actually a good one:
    // - for (i in 1..(n + 1))
    // It tells us we could improve the code by starting counting from 1,
    // since adding zero to the sum doesn't make much sense.
    fun sum_intermediate_in_for(n: u64): u64 {
        let sum = 0;

        for (i in 0..(n + 1)) {
            if (i % 10 == 0) continue;
            sum = sum + i
        };

        sum
    }

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

        assert!(sum_intermediate_in_for(0) == 0, 0);
        assert!(sum_intermediate_in_for(1) == 1, 0);
        assert!(sum_intermediate_in_for(2) == 3, 0);
        assert!(sum_intermediate_in_for(3) == 6, 0);
        assert!(sum_intermediate_in_for(4) == 10, 0);
        assert!(sum_intermediate_in_for(5) == 15, 0);
        assert!(sum_intermediate_in_for(6) == 21, 0);
        assert!(sum_intermediate_in_for(7) == 28, 0);
        assert!(sum_intermediate_in_for(8) == 36, 0);
        assert!(sum_intermediate_in_for(9) == 45, 0);
        assert!(sum_intermediate_in_for(10) == 45, 0);
        assert!(sum_intermediate_in_for(11) == 56, 0);
    }
}
