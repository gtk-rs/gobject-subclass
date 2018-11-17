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

extern crate gobject_subclass as subclass;
use subclass::object;
use subclass::prelude::*;

// FIXME
use subclass::properties::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    pub struct SimpleObject {
        name: RefCell<Option<String>>,
        parent: glib::WeakRef<super::SimpleObject>,
    }

    static PROPERTIES: [Property; 1] = [Property::String(
        "name",
        "Name",
        "Name",
        None,
        PropertyMutability::ReadWrite,
    )];

    // TODO: want default ones for this
    #[repr(C)]
    pub struct SimpleObjectInstance {
        parent: gobject_ffi::GObject,
    }

    impl InstanceStruct for SimpleObjectInstance {
        type Type = SimpleObject;
    }

    // TODO: want default ones for this
    #[repr(C)]
    pub struct SimpleObjectClass {
        parent_class: gobject_ffi::GObjectClass,
    }

    impl ClassStruct for SimpleObjectClass {
        type Type = SimpleObject;
    }

    // TODO: macro for these, like glib_wrapper!() or in it even
    unsafe impl IsAClass<object::ObjectClass> for SimpleObjectClass {}

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

    unsafe impl ObjectSubclass for SimpleObject {
        const NAME: &'static str = "SimpleObject";
        type ParentType = glib::Object;
        type Instance = SimpleObjectInstance;
        type Class = SimpleObjectClass;

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

        fn class_init(klass: &mut SimpleObjectClass) {
            object::ObjectClassExt::override_vfuncs(klass);

            klass.install_properties(&PROPERTIES);

            klass.add_signal("name-changed", &[String::static_type()], glib::Type::Unit);
        }

        fn new(obj: &glib::Object) -> Self {
            Self {
                name: RefCell::new(None),
                parent: obj
                    .downcast_ref::<super::SimpleObject>()
                    .unwrap()
                    .downgrade(),
            }
        }
    }

    impl ObjectImpl for SimpleObject {
        // TODO macro
        fn get_type_data(&self) -> ptr::NonNull<subclass::TypeData> {
            Self::type_data()
        }

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

        fn constructed(&self, obj: &glib::Object) {
            self.parent_constructed(obj);
        }
    }
}

glib_wrapper! {
    pub struct SimpleObject(Object<imp::SimpleObjectInstance, imp::SimpleObjectClass>);

    match fn {
        get_type => || imp::SimpleObject::get_type().to_glib(),
    }
}

impl ops::Deref for SimpleObject {
    type Target = imp::SimpleObject;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let stash = self.to_glib_none();
            let ptr: *mut imp::SimpleObjectInstance = stash.0;
            let imp = (*ptr).get_impl();
            imp
        }
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
        })
        .unwrap()
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
