// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

extern crate glib;

#[macro_use]
extern crate gobject_subclass as subclass;
use subclass::object;
use subclass::prelude::*;

pub struct SimpleObject {}

impl SimpleObject {
    object_get_type!();
}

impl ObjectSubclass for SimpleObject {
    const NAME: &'static str = "SimpleObject";
    type ParentType = glib::Object;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    object_subclass!();

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        ObjectClassExt::override_vfuncs(klass);
    }

    fn new(_obj: &glib::Object) -> Self {
        Self {}
    }
}

impl object::ObjectImpl for SimpleObject {
    object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }
}

#[test]
fn test_create() {
    let type_ = SimpleObject::get_type();
    let _obj = glib::Object::new(type_, &[]).unwrap();
}
