#![no_std]

extern crate alloc;
use alloc::format;
use dusk_data_driver::{ConvertibleContract, Error, JsonValue};

/// Data driver for `my-first-contract`:
///
/// Functions:
/// - read_value: input (), output u32
/// - increment: input (), output ()
/// - init: input u32, output ()
///
/// Events:
/// - "INIT": payload u32
#[derive(Default)]
pub struct ContractDriver;

impl ConvertibleContract for ContractDriver {
    fn encode_input_fn(&self, fn_name: &str, json: &str) -> Result<alloc::vec::Vec<u8>, Error> {
        match fn_name {
            "read_value" => dusk_data_driver::json_to_rkyv::<()>(&json),
            "increment"  => dusk_data_driver::json_to_rkyv::<()>(&json),
            "init"       => dusk_data_driver::json_to_rkyv::<u32>(&json),
            name => Err(Error::Unsupported(format!("fn_name {name}"))),
        }
    }

    fn decode_input_fn(&self, fn_name: &str, rkyv: &[u8]) -> Result<JsonValue, Error> {
        match fn_name {
            "read_value" => dusk_data_driver::rkyv_to_json::<()>(&rkyv),
            "increment"  => dusk_data_driver::rkyv_to_json::<()>(&rkyv),
            "init"       => dusk_data_driver::rkyv_to_json::<u32>(&rkyv),
            name => Err(Error::Unsupported(format!("fn_name {name}"))),
        }
    }

    fn decode_output_fn(&self, fn_name: &str, rkyv: &[u8]) -> Result<JsonValue, Error> {
        match fn_name {
            "read_value" => dusk_data_driver::rkyv_to_json::<u32>(&rkyv),
            "increment"  => dusk_data_driver::rkyv_to_json::<()>(&rkyv),
            "init"       => dusk_data_driver::rkyv_to_json::<()>(&rkyv),
            name => Err(Error::Unsupported(format!("fn_name {name}"))),
        }
    }

    fn decode_event(&self, event_name: &str, rkyv: &[u8]) -> Result<JsonValue, Error> {
        match event_name {
            "INIT" => dusk_data_driver::rkyv_to_json::<u32>(&rkyv),
            event  => Err(Error::Unsupported(format!("event {event}"))),
        }
    }

    fn get_schema(&self) -> alloc::string::String {
        alloc::string::String::from(include_str!("schema.json"))
    }
}

#[cfg(all(target_family = "wasm", feature = "ffi"))]
dusk_data_driver::generate_wasm_entrypoint!(ContractDriver);
