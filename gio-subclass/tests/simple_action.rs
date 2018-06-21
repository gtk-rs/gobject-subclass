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
extern crate gio_sys as gio_ffi;

use glib::value::AnyValue;

#[macro_use]
extern crate gobject_subclass;
use gobject_subclass::object::*;

#[macro_use]
extern crate gio_subclass;
use gio_subclass::action::*;

mod imp {
    use super::*;
    use std::cell::RefCell;

    static PROPERTIES: [Property; 5] = [
        Property::String(
            "name",
            "Action Name",
            "The name used to invoke the action",
            None,
            PropertyMutability::ReadWrite // | PropertyMutability::ConstructOnly
                // | PropertyMutability::StaticStrings,
        ),
        Property::Boxed(
            "parameter-type",
            "Parameter Type",
            "The type of GVariant passed to activate()",
            glib::VariantType::static_type,
            PropertyMutability::ReadWrite// |
            // PropertyMutability::ConstructOnly |
            // PropertyMutability::StaticStrings,
        ),
        Property::Boolean(
            "enabled",
            "Enabled",
            "If the action can be activated",
            true,
            PropertyMutability::ReadWrite
             // | PropertyMutability::StaticStrings,
        ),
        Property::Boxed(
            "state-type",
            "State Type",
            "The type of the state kept by the action",
            glib::VariantType::static_type,
            PropertyMutability::ReadWrite //|
            // PropertyMutability::ConstructOnly |
            // PropertyMutability::StaticStrings,
        ),
        Property::Variant(
            "state",
            "State",
            "The state the action is in",
            || glib::VariantType::new("*").unwrap(),
            None,
            PropertyMutability::ReadWrite //|
            //PropertyMutability::StaticStrings,
        ),
    ];

    #[derive(Default)]
    pub struct SimpleAction {
        name: RefCell<Option<String>>,
        parameter_type: RefCell<Option<glib::VariantType>>,
        state_type: RefCell<Option<glib::VariantType>>,
        enabled: RefCell<bool>,
        state: RefCell<Option<glib::Variant>>,
        state_hint: RefCell<Option<glib::Variant>>,
        state_set_already: RefCell<bool>,
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
            klass.install_properties(&PROPERTIES);
        }

        fn init(obj: &Object) -> Box<ObjectImpl<Object>> {
            let imp = SimpleAction{
                enabled: RefCell::new(true),
                ..SimpleAction::default()};
            Box::new(imp)
        }
    }

    impl ObjectImpl<Object> for SimpleAction {
        fn set_property(&self, obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => {
                    let name = value.get();
                    self.name.replace(name.clone());
                },
                Property::Boxed("parameter-type", ..) => {
                    let ptype: Option<glib::VariantType> = value.get();
                    self.parameter_type.replace(ptype.clone());
                },
                Property::Variant("state-type", ..) => {
                    let name = value.get();
                    self.state_type.replace(name.clone());
                },
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => Ok(self.name.borrow().clone().to_value()),
                Property::Boxed("parameter-type", ..) => Ok(self.parameter_type.borrow().clone().to_value()),
                Property::Boxed("state-type", ..) => Ok(self.state_type.borrow().clone().to_value()),
                _ => unimplemented!(),
            }
        }
    }

    impl ActionImpl for SimpleAction {
        fn activate(&self, action: &gio::Action, parameter: Option<&glib::Variant>) {
            unimplemented!();
        }

        fn change_state(&self, action: &gio::Action, value: &glib::Variant) {
            unimplemented!();
        }

        fn get_enabled(&self, action: &gio::Action) -> bool {
            *self.enabled.borrow()
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
        [Object => InstanceStruct<Object>,
         gio::Action => gio_ffi::GAction];

    match fn {
        get_type => || imp::SimpleAction::get_type().to_glib(),
    }
}

impl SimpleAction {
    pub fn new<'a, P: Into<Option<&'a glib::VariantTy>>>(
        name: &str,
        parameter_type: P,
    ) -> SimpleAction {
        use glib::object::Downcast;

        let ty: Option<glib::VariantType> = parameter_type.into().map(ToOwned::to_owned);

        unsafe {
            let obj = glib::Object::new(Self::static_type(), &[
                ("name", &name),
                ("parameter-type", &ty.to_value())
            ]);
            println!("obj{:?}", obj );
            obj.unwrap()
                .downcast_unchecked()
        }
    }

    pub fn new_stateful<'a, P: Into<Option<&'a glib::VariantTy>>>(
        name: &str,
        parameter_type: P,
        state: &glib::Variant,
    ) -> SimpleAction {
        use glib::object::Downcast;

        unsafe {
            glib::Object::new(Self::static_type(), &[
                ("name", &name),
                ("parameter-type", &parameter_type.into()),
                ("state", &state),
            ])
                .unwrap()
                .downcast_unchecked()
        }
    }
}

gobject_subclass_deref!(SimpleAction, Object);

#[test]
fn test_basic() {
    // let v = glib::VariantTy::new("*").unwrap();
    let action = SimpleAction::new("foo", None);

    assert!(action.get_enabled());
    //assert!(action.get_parameter_type().is_none());
    assert!(action.get_state_type().is_none());
    // assert!(action.get_state_hint().is_none());
    // assert!(action.get_state().is_none());


}
