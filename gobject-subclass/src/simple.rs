// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use glib;
use prelude::*;

/// A simple instance struct that does not store any additional data
#[repr(C)]
pub struct InstanceStruct<T: ObjectSubclass> {
    parent: <T::ParentType as glib::wrapper::Wrapper>::GlibType,
}

impl<T: ObjectSubclass> super::types::InstanceStruct for InstanceStruct<T> {
    type Type = T;
}

/// A simple class struct that does not store any additional data
/// or virtual methods
#[repr(C)]
pub struct ClassStruct<T: ObjectSubclass> {
    parent_class: <T::ParentType as glib::wrapper::Wrapper>::GlibClassType,
}

impl<T: ObjectSubclass> super::types::ClassStruct for ClassStruct<T> {
    type Type = T;
}

unsafe impl<T: ObjectSubclass, U: IsClassFor<T::ParentType>> IsAClass<U> for ClassStruct<T> {}
