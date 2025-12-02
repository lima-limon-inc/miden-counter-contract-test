// Do not link against libstd (i.e. anything defined in `std::`)
#![cfg_attr(not(test), no_std)]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;

use miden_harness_macros::miden_test_block;

#[cfg(not(test))]
mod component {
    use miden::{component, felt, Felt, StorageMap, StorageMapAccess, Word};

    /// Main contract structure for the counter example.
    #[component]
    struct CounterContract {
        /// Storage map holding the counter value.
        #[storage(slot(0), description = "counter contract storage map")]
        count_map: StorageMap,
    }

    #[component]
    impl CounterContract {
        /// Returns the current counter value stored in the contract's storage map.
        pub fn get_count(&self) -> Felt {
            let key = Word::from([felt!(0), felt!(0), felt!(0), felt!(1)]);
            self.count_map.get(&key)
        }

        /// Increments the counter value stored in the contract's storage map by one.
        pub fn increment_count(&self) -> Felt {
            let key = Word::from([felt!(0), felt!(0), felt!(0), felt!(1)]);
            let current_value: Felt = self.count_map.get(&key);
            let new_value = current_value + felt!(1);
            self.count_map.set(key, new_value);
            new_value
        }
    }
}

#[miden_test_block]
mod tests {
    use winter_utils::Deserializable;

    #[miden_test]
    fn bar() {
        std::dbg!(&package);
        assert_eq!(1, 1 + 1);
    }

    #[miden_test]
    fn foo() {
        assert_eq!(2, 1 + 1)
    }
}
