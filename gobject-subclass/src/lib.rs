// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

extern crate libc;

pub extern crate glib;

#[macro_use]
#[doc(hidden)]
pub mod guard;

pub mod simple;
pub mod types;

pub mod object;
pub mod properties;

pub mod prelude {
    //! Prelude that re-exports all important traits from this crate
    pub use super::object::{ObjectClassExt, ObjectImpl};
    pub use super::types::{ClassStruct, InstanceStruct, IsAClass, IsClassFor, ObjectSubclass};
}

pub use types::register_type;
pub use types::TypeData;
