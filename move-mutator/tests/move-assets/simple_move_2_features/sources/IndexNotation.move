module TestAccount::IndexNotation {
    fun get_element(v: &vector<u64>, i: u64): u64 {
        v[i]
    }

    fun sum_first_two(v: &vector<u64>): u64 {
        v[0] + v[1]
    }

    fun increment_element(v: &mut vector<u64>, i: u64) {
        *(&mut v[i]) = v[i] + 1;
    }

    fun is_element_greater(v: &vector<u64>, i: u64, threshold: u64): bool {
        v[i] > threshold
    }

    #[test]
    fun test_get_element() {
        let v = vector[10, 20, 30, 40];
        assert!(get_element(&v, 0) == 10);
        assert!(get_element(&v, 1) == 20);
        assert!(get_element(&v, 2) == 30);
        assert!(get_element(&v, 3) == 40);
    }

    #[test]
    fun test_sum_first_two() {
        let v = vector[5, 10, 15];
        assert!(sum_first_two(&v) == 15);

        let v2 = vector[100, 200, 300];
        assert!(sum_first_two(&v2) == 300);
    }

    #[test]
    fun test_increment_element() {
        let v = vector[10, 20, 30];
        increment_element(&mut v, 0);
        assert!(v[0] == 11);
        assert!(v[1] == 20);

        increment_element(&mut v, 1);
        assert!(v[1] == 21);
    }

    #[test]
    fun test_is_element_greater() {
        let v = vector[10, 20, 30];
        assert!(is_element_greater(&v, 0, 5) == true);
        assert!(is_element_greater(&v, 0, 10) == false);
        assert!(is_element_greater(&v, 2, 25) == true);
        assert!(is_element_greater(&v, 2, 30) == false);
    }
}
