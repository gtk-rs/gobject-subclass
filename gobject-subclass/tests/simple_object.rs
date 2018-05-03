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
use std::sync::{Once, ONCE_INIT};

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;
use glib::prelude::*;
use glib::translate::*;

extern crate gobject_subclass;
use gobject_subclass::object::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    pub struct SimpleObject {
        name: RefCell<Option<String>>,
    }

    static PROPERTIES: [Property; 1] = [Property::String(
        "name",
        "Name",
        "Name",
        None,
        PropertyMutability::ReadWrite,
    )];

    impl SimpleObject {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let t = register_type(SimpleObjectStatic);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut ObjectClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_obj: &Object) -> Box<ObjectImpl<Object>> {
            let imp = Self {
                name: RefCell::new(None),
            };
            Box::new(imp)
        }

        pub fn set_name(&self, name: Option<&str>) {
            self.name.replace(name.map(String::from));
        }

        pub fn get_name(&self) -> Option<String> {
            self.name.borrow().clone()
        }
    }

    impl ObjectImpl<Object> for SimpleObject {
        fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => {
                    self.name.replace(value.get());
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => Ok(self.name.borrow().clone().to_value()),
                _ => unimplemented!(),
            }
        }
    }

    struct SimpleObjectStatic;

    impl ImplTypeStatic<Object> for SimpleObjectStatic {
        fn get_name(&self) -> &str {
            "SimpleObject"
        }

        fn new(&self, obj: &Object) -> Box<ObjectImpl<Object>> {
            SimpleObject::init(obj)
        }

        fn class_init(&self, klass: &mut ObjectClass) {
            SimpleObject::class_init(klass);
        }
    }
}

glib_wrapper! {
    pub struct SimpleObject(Object<imp::SimpleObject>):
        [Object => InstanceStruct<Object>];

    match fn {
        get_type => || imp::SimpleObject::get_type().to_glib(),
    }
}

impl SimpleObject {
    pub fn new(name: Option<&str>) -> SimpleObject {
        use glib::object::Downcast;

        unsafe {
            glib::Object::new(Self::static_type(), &[("name", &name)])
                .unwrap()
                .downcast_unchecked()
        }
    }
}

// TODO: This one should probably get a macro
impl ops::Deref for SimpleObject {
    type Target = imp::SimpleObject;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let base: Object = from_glib_borrow(self.to_glib_none().0);
            let imp = base.get_impl();
            let imp = imp.downcast_ref::<imp::SimpleObject>().unwrap();
            // Cast to a raw pointer to get us an appropriate lifetime: the compiler
            // can't know that the lifetime of base is the same as the one of self
            &*(imp as *const imp::SimpleObject)
        }
    }
}

#[test]
fn test_create() {
    let obj = SimpleObject::new(None);
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), None);
    assert_eq!(obj.get_name(), None);
}

#[test]
fn test_create_with_name() {
    let obj = SimpleObject::new(Some("meh"));
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), Some("meh"));
    assert_eq!(obj.get_name(), Some("meh".into()));
}

#[test]
fn test_set_property() {
    let obj = SimpleObject::new(None);
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), None);
    assert_eq!(obj.get_name(), None);

    obj.set_property("name", &"meh").unwrap();
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), Some("meh"));
    assert_eq!(obj.get_name(), Some("meh".into()));

    obj.set_name(Some("bah"));
    assert_eq!(obj.get_property("name").unwrap().get::<&str>(), Some("bah"));
    assert_eq!(obj.get_name(), Some("bah".into()));
}
