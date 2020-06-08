/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

//! [TVM](https://github.com/apache/incubator-tvm) is a compiler stack for deep learning systems.
//!
//! This crate provides an idiomatic Rust API for TVM runtime frontend.
//!
//! One particular use case is that given optimized deep learning model artifacts,
//! (compiled with TVM) which include a shared library
//! `lib.so`, `graph.json` and a byte-array `param.params`, one can load them
//! in Rust idomatically to create a TVM Graph Runtime and
//! run the model for some inputs and get the
//! desired predictions *all in Rust*.
//!
//! Checkout the `examples` repository for more details.

// NB: required for in-crate uses of the procedural macros.
pub use crate as tvm_rt;

pub mod object;
pub mod string;

pub use object::*;
pub use string::*;

use std::{
    ffi::{CStr, CString},
    str,
};

pub use crate::{
    context::{Context, DeviceType},
    errors::*,
    function::Function,
    module::Module,
    ndarray::NDArray,
};

pub use function::{ArgValue, RetValue};
pub use tvm_sys::byte_array::ByteArray;
pub use tvm_sys::datatype::DataType;
use tvm_sys::ffi;

pub use tvm_macros::external;

// Macro to check the return call to TVM runtime shared library.
#[macro_export]
macro_rules! check_call {
    ($e:expr) => {{
        if unsafe { $e } != 0 {
            panic!("{}", $crate::get_last_error());
        }
    }};
}

/// Gets the last error message.
pub fn get_last_error() -> &'static str {
    unsafe {
        match CStr::from_ptr(ffi::TVMGetLastError()).to_str() {
            Ok(s) => s,
            Err(_) => "Invalid UTF-8 message",
        }
    }
}

pub(crate) fn set_last_error<E: std::error::Error>(err: &E) {
    let c_string = CString::new(err.to_string()).unwrap();
    unsafe {
        ffi::TVMAPISetLastError(c_string.as_ptr());
    }
}

#[macro_use]
pub mod function;
pub mod context;
pub mod errors;
pub mod module;
pub mod ndarray;
pub mod to_boxed_fn;
pub mod to_function;
pub mod value;

/// Outputs the current TVM version.
pub fn version() -> &'static str {
    match str::from_utf8(ffi::TVM_VERSION) {
        Ok(s) => s,
        Err(_) => "Invalid UTF-8 string",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_version() {
        println!("TVM version: {}", version());
    }

    #[test]
    fn set_error() {
        let err = errors::EmptyArrayError;
        set_last_error(&err.into());
        assert_eq!(get_last_error().trim(), errors::EmptyArrayError.to_string());
    }
}
