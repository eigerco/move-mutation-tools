module TestAccount::ReceiverFuncs {
    struct S has drop { x: u64, y: u64 }

    fun sum(self: &S): u64 { 
		self.x + self.y
	}

    #[test]
    fun sum_valid() {
        assert!(S { x: 0, y: 0 }.sum() == 0);
        assert!(S { x: 1, y: 0 }.sum() == 1);
        assert!(S { x: 0, y: 1 }.sum() == 1);
        assert!(S { x: 5, y: 5 }.sum() == 10);
        assert!(S { x: 50, y: 10 }.sum() == 60);
    }
}
