// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;
use std::sync::{Once, ONCE_INIT};

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;
use glib::prelude::*;
use glib::translate::*;
extern crate gio;
use gio::prelude::*;

#[macro_use]
extern crate gobject_subclass;
use gobject_subclass::object::*;

#[macro_use]
extern crate gio_subclass;
use gio_subclass::action::*;

mod imp {
    use super::*;

    pub struct SimpleAction {
        dummy: i32,
    }

    impl SimpleAction {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let t = register_type(SimpleActionStatic);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut ObjectClass) {
        }

        fn init(obj: &Object) -> Box<ObjectImpl<Object>> {
            let imp = Self {
                dummy: 0,
            };
            Box::new(imp)
        }
    }

    impl ObjectImpl<Object> for SimpleAction {
    }

    impl ActionImpl for SimpleAction {
        fn activate(&self, action: &gio::Action, parameter: Option<&glib::Variant>) {
            unimplemented!();
        }

        fn change_state(&self, action: &gio::Action, value: &glib::Variant) {
            unimplemented!();
        }

        fn get_enabled(&self, action: &gio::Action) -> bool {
            unimplemented!();
        }

        fn get_name(&self, action: &gio::Action) -> Option<String> {
            unimplemented!();
        }

        fn get_parameter_type(&self, action: &gio::Action) -> Option<glib::VariantType> {
            unimplemented!();
        }

        fn get_state(&self, action: &gio::Action) -> Option<glib::Variant> {
            unimplemented!();
        }

        fn get_state_hint(&self, action: &gio::Action) -> Option<glib::Variant> {
            unimplemented!();
        }

        fn get_state_type(&self, action: &gio::Action) -> Option<glib::VariantType> {
            unimplemented!();
        }
    }

    struct SimpleActionStatic;

    impl ImplTypeStatic<Object> for SimpleActionStatic {
        fn get_name(&self) -> &str {
            "SimpleAction"
        }

        fn new(&self, obj: &Object) -> Box<ObjectImpl<Object>> {
            SimpleAction::init(obj)
        }

        fn class_init(&self, klass: &mut ObjectClass) {
            SimpleAction::class_init(klass);
        }

        fn type_init(&self, token: &TypeInitToken, type_: glib::Type) {
            register_action(token, type_, self);
        }
    }

    impl ActionImplStatic<Object> for SimpleActionStatic {
        fn get_impl<'a>(&self, imp: &'a Box<ObjectImpl<Object>>) -> &'a ActionImpl {
            imp.downcast_ref::<SimpleAction>().unwrap()
        }
    }
}

glib_wrapper! {
    pub struct SimpleAction(Object<imp::SimpleAction>):
        [Object => InstanceStruct<Object>];

    match fn {
        get_type => || imp::SimpleAction::get_type().to_glib(),
    }
}

impl SimpleAction {
    pub fn new() -> SimpleAction {
        use glib::object::Downcast;

        unsafe {
            glib::Object::new(Self::static_type(), &[])
                .unwrap()
                .downcast_unchecked()
        }
    }
}

gobject_subclass_deref!(SimpleAction, Object);

#[test]
fn test_create() {
    let obj = SimpleAction::new();
}
