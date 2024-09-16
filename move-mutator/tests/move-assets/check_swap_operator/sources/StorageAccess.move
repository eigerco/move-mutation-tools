module 0x42::m1 {
    struct T has copy,key,drop { a: u64 }

    public fun create_t(account: &signer) {
        let t = T { a: 0 };
        move_to<T>(account, t);
    }

    public fun ret_t_ref(a: address): u64 acquires T {
        let ref = borrow_global<T>(a).a;
        let retval = ref + 5;
        retval
    }

    public fun ret_t_mut_ref(a: address): u64 acquires T {
        let ref = &mut borrow_global_mut<T>(a).a;
        *ref = *ref + 1;
        let retval = *ref + 10;
        retval
    }
}

module 0x42::m2 {
    public fun borrow_then_borrow_and_modify(a: address): u64 {
        0x42::m1::ret_t_ref(a) + 0x42::m1::ret_t_mut_ref(a)
    }

    public fun borrow_then_borrow_and_modify_reversed(a: address): u64 {
       0x42::m1::ret_t_mut_ref(a) + 0x42::m1::ret_t_ref(a)
    }

    #[test(arg = @0xC0FFEE)]
    fun run(arg: signer) {
        0x42::m1::create_t(&arg);
        assert!(borrow_then_borrow_and_modify(0x1::signer::address_of(&arg)) == 16, 0);
    }

    #[test(arg = @0xC0FFEE)]
    fun run_reverse(arg: signer) {
        0x42::m1::create_t(&arg);
        assert!(borrow_then_borrow_and_modify_reversed(0x1::signer::address_of(&arg)) == 17, 0);
    }
}

// Info 1: All functions below use binary operators `+`, `*`, `&`, `&&`, `|`, `||`, `==`, `!=`, `^`
// for which the order doesn't matter.
//
// Info 2: Deliberately use funny identation for the sake of the extra test.
module 0x42::m3 {
    fun swap_op_should_mutate_once_v1(a: address, b: u64): u64 {
       0x42::m1::ret_t_mut_ref(a)     * b
    }

    fun swap_op_should_mutate_once_v2(a: address, b: u64): u64 {
        b |0x42::m1::ret_t_ref(a)
    }

    inline fun swap_op_should_mutate_once_v3(a: u32, f: |u32| u32): u32 {
        // Expect a swap here:
        a + f(3)
    }

    inline fun swap_op_should_mutate_seven_times(a: address, b: u64, closure: |u64| bool): u64 {
       let b1 = false;
       let b2 = false;

       // Expect no swaps here:
       if (b1 || b2) return 0;
       if (b1 &&    b2) return 0;
       if (b1!= true) return 0;

       // Expect one swap here for each:
       if (b1 == closure(23)) return 1;
       if (closure(20 ) !=    closure(23)) return 2;
       if (!closure(22) &&true) return   3;
       if (closure(2) || false) return 4;

       // Expect two swaps here:
       if (b2 !=    (0x42::m1::ret_t_ref(a)==  1)) return 4;

       // One more swap:
       (b^10) + 0x42::m2::borrow_then_borrow_and_modify_reversed(a)
    }

    fun swap_op_should_mutate_three_times(a: address, b: u64): u64 {
       (7 *   (0x42::m1::ret_t_mut_ref(a)+ b)) & 259
    }

    fun swap_op_should_not_mutate(a: u64, b: u64): u64 {
       let c = 2 * (a +b);
       let d = c* (40& (a | b));

       if (c ==d) {
           c + d
       } else {
           d  +c
       }
    }
}
