// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! # Counter Contract
//!
//! This is a simple counter contract that increments a counter by 1 every time
//! it is called. The contract has two functions:
//! - `read_value` which reads the current value of the counter
//! - `increment` which increments the counter by 1
//! - `init` which initializes the counter with a given value during deployment
//!
//! This contract does not use any macros or frameworks which potentially
//! abstract the underlying details for writing a contract on Dusk

#![no_std]

use rusk_abi::wrap_call;
use contract::Counter;

/// This is the actual state of the contract that stores the values.
/// It is a mutable static variable that is initialized to 0 during
/// compilation.
static mut STATE: Counter = Counter { value: 0 };

mod contract {
    use rkyv::{Archive, Serialize};

    /// The Counter struct represents the values that the contract will store in
    /// its state.
    #[derive(Debug, Archive, Serialize)]
    pub struct Counter {
        pub value: u32,
    }

    impl Counter {
        /// Read the value of the counter.
        pub fn read_value(&self) -> u32 {
            self.value
        }

        /// Increment the value of the counter by 1.
        /// Note that this function is &mut self, so it mutates the state once
        /// it is called from a transaction.
        pub fn increment(&mut self) {
            self.value += 1
        }

        /// The init() function acts as an initializer for the contract and is
        /// called when the contract is deployed on Dusk.
        pub fn init(&mut self, value: u32) {
            self.value = value;
            rusk_abi::emit("INIT", value); // One can also emit events here
        }
    }
}

#[no_mangle]
unsafe fn read_value(arg_len: u32) -> u32 {
    wrap_call(arg_len, |_: ()| STATE.read_value())
}

#[no_mangle]
unsafe fn increment(arg_len: u32) -> u32 {
    wrap_call(arg_len, |_: ()| STATE.increment())
}

#[no_mangle]
unsafe fn init(arg_len: u32) -> u32 {
    wrap_call(arg_len, |arg: u32| STATE.init(arg))
}
