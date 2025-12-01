// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

use miden_harness_macros::miden_test_block;

#[cfg(not(test))]
mod component {
    use miden::*;

    use self::bindings::miden::counter_contract::counter_contract;

    #[note_script]
    fn run(_arg: Word) {
        let initial_value = counter_contract::get_count();
        counter_contract::increment_count();
        let expected_value = initial_value + Felt::from_u32(1);
        let final_value = counter_contract::get_count();
        assert_eq(final_value, expected_value);
    }
}

#[miden_test_block]
mod tests {
    #[miden_test]
    fn bar() {
        assert_eq!(1, 1 + 1)
    }

    #[miden_test]
    fn foo() {
        assert_eq!(2, 1 + 1)
    }
}
