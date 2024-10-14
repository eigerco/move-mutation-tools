module TestAccount::BasicOps {
    public fun sum(x: u64, y: u64): u64 {
        let sum_r = x + y;

        spec {
                assert sum_r == x + y;
        };

        sum_r
    }

    #[test]
    fun sum_valid_usage_test() {
        assert!(sum(0, 0) == 0, 0);
        assert!(sum(1, 0) == 1, 0);
        assert!(sum(0, 1) == 1, 0);
        assert!(sum(5, 5) == 10, 0);
        assert!(sum(50, 10) == 60, 0);
    }

    #[mutation::skip]
    fun skip_sub(x: u64, y: u64): u64 {
        let sub_r = x - y;

        spec {
                assert sub_r == x - y;
        };

        sub_r
    }

    #[test]
    fun sub_valid_usage_test() {
        assert!(skip_sub(0, 0) == 0, 0);
        assert!(skip_sub(1, 0) == 1, 0);
        assert!(skip_sub(5, 5) == 0, 0);
        assert!(skip_sub(50, 10) == 40, 0);
    }

    #[test, expected_failure]
    fun sub_invalid_usage_test() {
        assert!(skip_sub(0, 5) == 0, 0);
    }
}

#[mutation::skip]
module TestAccount::SkipSum {
    fun skip_sum(x: u64, y: u64): u64 {
        TestAccount::BasicOps::sum(x, y) + 0
    }

    #[test]
    fun skip_sum_valid_usage_test() {
        assert!(skip_sum(0, 0) == 0, 0);
        assert!(skip_sum(1, 0) == 1, 0);
        assert!(skip_sum(0, 1) == 1, 0);
        assert!(skip_sum(5, 5) == 10, 0);
        assert!(skip_sum(50, 10) == 60, 0);
    }
}
