// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;
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
            klass.add_signal(
                "activate",
                &[glib::Variant::static_type()],
                glib::Type::Unit,
            );
            klass.add_signal("change-state", &[], glib::Type::Unit);
        }

        fn init(obj: &Object) -> Box<ObjectImpl<Object>> {
            let imp = SimpleAction {
                enabled: RefCell::new(true),
                ..SimpleAction::default()
            };
            Box::new(imp)
        }
    }

    impl ObjectImpl<Object> for SimpleAction {
        fn set_property(&self, obj: &glib::Object, id: u32, value: &glib::Value) {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => {
                    self.name.replace(value.get().clone());
                }
                Property::Boolean("enabled", ..) => {
                    self.enabled.replace(value.get().unwrap_or(false));
                }
                Property::Boxed("parameter-type", ..) => {
                    self.parameter_type.replace(value.get().clone());
                }
                Property::Boxed("state-type", ..) => {
                    self.state_type.replace(value.get().clone());
                }
                Property::Variant("state", ..) => {
                    self.state.replace(value.get().clone());
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id as usize];

            match *prop {
                Property::String("name", ..) => Ok(self.name.borrow().clone().to_value()),
                Property::Boolean("enabled", ..) => Ok(self.enabled.borrow().to_value()),
                Property::Boxed("parameter-type", ..) => {
                    Ok(self.parameter_type.borrow().clone().to_value())
                }
                Property::Boxed("state-type", ..) => {
                    Ok(self.state_type.borrow().clone().to_value())
                }
                Property::Variant("state", ..) => Ok(self.state.borrow().clone().to_value()),
                _ => unimplemented!(),
            }
        }
    }

    impl ActionImpl for SimpleAction {
        fn activate(&self, action: &gio::Action, parameter: Option<&glib::Variant>) {

            match *self.parameter_type.borrow(){
                None => assert!(parameter.is_none()),
                Some(ref t) => {
                    assert!(parameter.is_some());
                    assert!(parameter.unwrap().type_() == t);
                }
            };

            if !*self.enabled.borrow() {
                return;
            }

            /* If the user connected a signal handler then they are responsible
             * for handling activation.
             */

            /* If not, do some reasonable defaults for stateful actions. */

            match parameter{
                Some(p) => { action.emit("activate", &[p]).unwrap(); },

                None => {
                    //FIXME: how can we propagate the optional nature of the arg?
                    //       the value created here seems dangling for glib, and panics
                    action.emit("activate", &[&glib::Variant::from(false)]).unwrap();
                }
            };

        }

        fn change_state(&self, action: &gio::Action, value: &glib::Variant) {
            unimplemented!();
        }

        fn get_enabled(&self, action: &gio::Action) -> bool {
            *self.enabled.borrow()
        }

        fn get_name(&self, action: &gio::Action) -> Option<String> {
            self.name.borrow().clone()
        }

        fn get_parameter_type(&self, action: &gio::Action) -> Option<glib::VariantType> {
            self.parameter_type.borrow().clone()
        }

        fn get_state(&self, action: &gio::Action) -> Option<glib::Variant> {
            self.state.borrow().clone()
        }

        fn get_state_hint(&self, action: &gio::Action) -> Option<glib::Variant> {
            self.state_hint.borrow().clone()
        }

        fn get_state_type(&self, action: &gio::Action) -> Option<glib::VariantType> {
            self.state_type.borrow().clone()
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
            glib::Object::new(
                Self::static_type(),
                &[("name", &name), ("parameter-type", &ty.to_value())],
            ).unwrap()
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
            glib::Object::new(
                Self::static_type(),
                &[
                    ("name", &name),
                    ("parameter-type", &parameter_type.into()),
                    ("state", &state),
                ],
            ).unwrap()
                .downcast_unchecked()
        }
    }

    pub fn connect_activate<F: Fn(&Self, Option<glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        // FIXME: This needs some simplification...
        //
        // Get us Send/Sync constraints
        let f: Box<Fn(&Self, Option<glib::Variant>) + Send + Sync + 'static> = unsafe {
            let f: Box<Fn(&Self, Option<glib::Variant>) + 'static> = Box::new(f);
            Box::from_raw(Box::into_raw(f) as *mut _)
        };

        self.connect("activate", false, move |values| {
            use glib::object::Downcast;

            let obj: Self = unsafe {
                values[0]
                    .get::<glib::Object>()
                    .unwrap()
                    .downcast_unchecked()
            };

            let param = if values.len() > 1 { values[1].get::<glib::Variant>() } else {None};
            f(&obj, param);

            None
        }).unwrap()
    }

}

gobject_subclass_deref!(SimpleAction, Object);

#[test]
fn test_basic() {
    let action = SimpleAction::new("foo", None);

    assert!(action.get_enabled());
    assert!(action.get_parameter_type().is_none());
    assert!(action.get_state_type().is_none());
    assert!(action.get_state_hint().is_none());
    assert!(action.get_state().is_none());

    assert!(action.get_property("name").unwrap().get::<String>() == Some("foo".to_string()));
    assert!(
        action
            .get_property("parameter-type")
            .unwrap()
            .get::<glib::VariantType>()
            .is_none()
    );
    assert!(
        action
            .get_property("state-type")
            .unwrap()
            .get::<glib::VariantType>()
            .is_none()
    );
    assert!(
        action
            .get_property("state")
            .unwrap()
            .get::<glib::Variant>()
            .is_none()
    );

    assert!(action.get_property("enabled").unwrap().get::<bool>().unwrap());

    let did_run = Rc::new(RefCell::new(false));
    let dr = did_run.clone();

    action.connect_activate(move |_obj, param| {
        *dr.borrow_mut() = true;
    });
    assert!(!*did_run.borrow());

    action.activate(None);
    assert!(*did_run.borrow());

    *did_run.borrow_mut() = false;

}
