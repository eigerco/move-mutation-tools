module TestAccount::FriendVisibility {
    friend TestAccount::FriendHelper;

    public(friend) fun multiply(a: u64, b: u64): u64 {
        a * b
    }

    friend fun divide(a: u64, b: u64): u64 {
        a / b
    }

    public(friend) fun is_greater(a: u64, b: u64): bool {
        a > b
    }

    #[test]
    fun test_multiply() {
        assert!(multiply(5, 10) == 50);
        assert!(multiply(0, 100) == 0);
        assert!(multiply(7, 7) == 49);
    }

    #[test]
    fun test_divide() {
        assert!(divide(10, 2) == 5);
        assert!(divide(100, 10) == 10);
        assert!(divide(7, 2) == 3);
    }

    #[test]
    fun test_is_greater() {
        assert!(is_greater(10, 5) == true);
        assert!(is_greater(5, 10) == false);
        assert!(is_greater(5, 5) == false);
    }
}

module TestAccount::FriendHelper {
    use TestAccount::FriendVisibility;

    fun compute_area(width: u64, height: u64): u64 {
        FriendVisibility::multiply(width, height)
    }

    fun average(total: u64, count: u64): u64 {
        FriendVisibility::divide(total, count)
    }

    fun max(a: u64, b: u64): u64 {
        if (FriendVisibility::is_greater(a, b)) { a } else { b }
    }

    #[test]
    fun test_compute_area() {
        assert!(compute_area(5, 10) == 50);
        assert!(compute_area(10, 10) == 100);
    }

    #[test]
    fun test_average() {
        assert!(average(100, 10) == 10);
        assert!(average(50, 2) == 25);
    }

    #[test]
    fun test_max() {
        assert!(max(10, 5) == 10);
        assert!(max(5, 10) == 10);
        assert!(max(7, 7) == 7);
    }
}
