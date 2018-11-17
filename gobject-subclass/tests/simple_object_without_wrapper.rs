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

extern crate gobject_subclass as subclass;
use subclass::object;
use subclass::prelude::*;

pub struct SimpleObject {}

impl SimpleObject {
    // TODO This should be a macro
    pub fn get_type() -> glib::Type {
        use std::sync::Once;
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            subclass::register_type::<SimpleObject>();
        });

        Self::static_type()
    }
}

unsafe impl ObjectSubclass for SimpleObject {
    const NAME: &'static str = "SimpleObject";
    type ParentType = glib::Object;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    // TODO: macro
    fn type_data() -> ptr::NonNull<subclass::TypeData> {
        static mut DATA: subclass::TypeData = subclass::TypeData {
            type_: glib::Type::Invalid,
            parent_class: ptr::null_mut(),
            interfaces: ptr::null_mut(),
            private_offset: 0,
        };

        unsafe { ptr::NonNull::new_unchecked(&mut DATA) }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        ObjectClassExt::override_vfuncs(klass);
    }

    fn new(_obj: &glib::Object) -> Self {
        Self {}
    }
}

impl object::ObjectImpl for SimpleObject {
    // TODO macro
    fn get_type_data(&self) -> ptr::NonNull<subclass::TypeData> {
        Self::type_data()
    }

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }
}

#[test]
fn test_create() {
    let type_ = SimpleObject::get_type();
    let _obj = glib::Object::new(type_, &[]).unwrap();
}
