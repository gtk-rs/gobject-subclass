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

#[macro_use]
extern crate gobject_subclass;
use gobject_subclass::object::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    pub struct SimpleObject {
        name: RefCell<Option<String>>,
        parent: glib::WeakRef<Object>,
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

            klass.add_signal("name-changed", &[String::static_type()], glib::Type::Unit);
        }

        fn init(obj: &Object) -> Box<ObjectImpl<Object>> {
            let imp = Self {
                name: RefCell::new(None),
                parent: obj.downgrade(),
            };
            Box::new(imp)
        }

        pub fn set_name(&self, name: Option<&str>) {
            self.name.replace(name.map(String::from));
            self.parent
                .upgrade()
                .map(|o| o.emit("name-changed", &[&name]).unwrap());
        }

        pub fn get_name(&self) -> Option<String> {
            self.name.borrow().clone()
        }
    }

    impl ObjectImpl<Object> for SimpleObject {
        fn set_property(&self, obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => {
                    let name = value.get();
                    self.name.replace(name.clone());
                    obj.emit("name-changed", &[&name]).unwrap();
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

    pub fn connect_name_changed<F: Fn(&Self, Option<&str>) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        // FIXME: This needs some simplification...
        //
        // Get us Send/Sync constraints
        let f: Box<Fn(&Self, Option<&str>) + Send + Sync + 'static> = unsafe {
            let f: Box<Fn(&Self, Option<&str>) + 'static> = Box::new(f);
            Box::from_raw(Box::into_raw(f) as *mut _)
        };

        self.connect("name-changed", false, move |values| {
            use glib::object::Downcast;

            let obj: Self = unsafe {
                values[0]
                    .get::<glib::Object>()
                    .unwrap()
                    .downcast_unchecked()
            };

            let name = values[1].get();

            f(&obj, name);

            None
        }).unwrap()
    }
}

gobject_subclass_deref!(SimpleObject, Object);

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

#[test]
fn test_signals() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let obj = SimpleObject::new(None);

    let name = Rc::new(RefCell::new(None));
    let name_clone = name.clone();

    obj.connect_name_changed(move |_obj, name| {
        name_clone.replace(name.map(String::from));
    });

    assert_eq!(*name.borrow(), None);
    obj.set_property("name", &"meh").unwrap();
    assert_eq!(*name.borrow(), Some(String::from("meh")));
}
