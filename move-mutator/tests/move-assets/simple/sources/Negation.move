module TestAccount::Negation {
    fun neg_log(x: bool): bool {
        !x
    }

    #[test]
    fun neg_log_test() {
        assert!(neg_log(true) == false, 0);
        assert!(neg_log(false) == true, 0);
    }

    spec neg_log {
        ensures result == !x;
    }
}
