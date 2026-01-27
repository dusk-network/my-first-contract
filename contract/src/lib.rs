// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! # Counter Contract
//!
//! This is a simple counter contract that increments a counter by 1 every time
//! it is called. The contract has three functions:
//! - `read_value` which reads the current value of the counter
//! - `increment` which increments the counter by 1
//! - `init` which initializes the counter with a given value during deployment
//!
//! This contract uses the `#[contract]` macro from `dusk-forge` to auto-generate
//! extern wrappers, schema, and data-driver implementations.

#![no_std]

/// The Counter contract module.
///
/// The `#[dusk_forge::contract]` macro generates:
/// - Static `STATE` variable with the contract struct
/// - Extern "C" wrapper functions for WASM export
/// - `CONTRACT_SCHEMA` constant with metadata
/// - `data_driver` module when compiled with the `data-driver` feature
#[dusk_forge::contract]
mod counter {
    /// The Counter struct represents the values that the contract will store in
    /// its state.
    pub struct Counter {
        value: u32,
    }

    impl Counter {
        /// Creates a new Counter instance with initial value 0.
        pub const fn new() -> Self {
            Self { value: 0 }
        }

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
        }
    }
}
