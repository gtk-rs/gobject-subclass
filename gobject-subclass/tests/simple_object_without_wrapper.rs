// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ops;
use std::ptr;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;
use glib::prelude::*;
use glib::translate::*;

extern crate gobject_subclass;
use gobject_subclass::object::*;
use gobject_subclass::properties::*;

use std::cell::RefCell;

pub struct SimpleObject {
    dummy: u32,
}

impl SimpleObject {
    // TODO This should be a macro
    pub fn get_type() -> glib::Type {
        use std::sync::Once;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            register_type::<SimpleObject>();
        });

        Self::static_type()
    }
}

unsafe impl ObjectSubclass for SimpleObject {
    const NAME: &'static str = "SimpleObject";
    type ParentType = glib::Object;
    type Instance = SimpleInstanceStruct<Self>;
    type Class = SimpleClassStruct<Self>;

    // TODO: macro
    fn type_data() -> ptr::NonNull<TypeData> {
        static mut DATA: TypeData = TypeData {
            type_: glib::Type::Invalid,
            parent_class: ptr::null_mut(),
            interfaces: ptr::null_mut(),
            private_offset: 0,
        };

        unsafe { ptr::NonNull::new_unchecked(&mut DATA) }
    }

    fn class_init(klass: &mut SimpleClassStruct<Self>) {
        ObjectClassExt::override_vfuncs(klass);
    }

    fn new(obj: &glib::Object) -> Self {
        Self { dummy: 0 }
    }
}

impl ObjectImpl for SimpleObject {
    // TODO macro
    fn get_type_data(&self) -> ptr::NonNull<TypeData> {
        Self::type_data()
    }

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }
}

#[test]
fn test_create() {
    let type_ = SimpleObject::get_type();
    let obj = glib::Object::new(type_, &[]).unwrap();
}
