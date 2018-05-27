// adaptation of https://gitlab.gnome.org/GNOME/glib/blob/master/gio/tests/gapplication-example-cmdline2.c
use std::mem;
use std::ops;
use std::ptr;
use std::sync::{Once, ONCE_INIT};

extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;
extern crate gio;

use gio::prelude::*;
use glib::prelude::*;
use glib::translate::*;

extern crate gio_subclass;

#[macro_use]
extern crate gobject_subclass;

use gio_subclass::application::*;
use gobject_subclass::object::*;

const EXIT_STATUS: i32 = 20;

// TODO: Wrap this in a macro
mod imp {
    use super::*;

    pub struct SimpleApplication;

    static PROPERTIES: [Property; 0] = [];

    impl SimpleApplication {
        pub fn get_type() -> glib::Type {
            static ONCE: Once = ONCE_INIT;
            static mut TYPE: glib::Type = glib::Type::Invalid;

            ONCE.call_once(|| {
                let t = register_type(SimpleApplicationStatic);
                unsafe {
                    TYPE = t;
                }
            });

            unsafe { TYPE }
        }

        fn class_init(klass: &mut ApplicationClass) {
            klass.install_properties(&PROPERTIES);
        }

        fn init(_application: &Application) -> Box<ApplicationImpl<Application>> {
            let imp = Self {};
            Box::new(imp)
        }
    }

    impl ObjectImpl<Application> for SimpleApplication {}

    impl ApplicationImpl<Application> for SimpleApplication {
        fn local_command_line(
            &self,
            _application: &Application,
            arguments: &mut ArgumentList,
        ) -> Option<i32> {
            let mut rm = Vec::new();

            for (i, line) in arguments.iter().enumerate() {
                if line.starts_with("--local-") {
                    rm.push(i)
                }
            }

            rm.reverse();

            for i in rm.iter() {
                arguments.remove(*i);
            }

            None
        }

        fn command_line(
            &self,
            _application: &Application,
            cmd_line: &gio::ApplicationCommandLine,
        ) -> i32 {
            let arguments = cmd_line.get_arguments();

            for arg in arguments {
                assert!(!arg.starts_with("--local-"))
            }

            return EXIT_STATUS;
        }
    }

    struct SimpleApplicationStatic;

    impl ImplTypeStatic<Application> for SimpleApplicationStatic {
        fn get_name(&self) -> &str {
            "SimpleApplication"
        }

        fn new(&self, application: &Application) -> Box<ApplicationImpl<Application>> {
            SimpleApplication::init(application)
        }

        fn class_init(&self, klass: &mut ApplicationClass) {
            SimpleApplication::class_init(klass);
        }
    }
}

glib_wrapper! {
    pub struct SimpleApplication(Object<imp::SimpleApplication>):
        [Application => InstanceStruct<Application>,
         gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap];


    match fn {
        get_type => || imp::SimpleApplication::get_type().to_glib(),
    }
}

gobject_subclass_deref!(SimpleApplication, Application);


impl SimpleApplication {
    pub fn new<'a, I: Into<Option<&'a str>>>(
        application_id: I,
        flags: gio::ApplicationFlags,
    ) -> Result<SimpleApplication, glib::BoolError> {
        use glib::object::Downcast;

        unsafe {
            match glib::Object::new(
                Self::static_type(),
                &[
                    ("application-id", &application_id.into()),
                    ("flags", &flags),
                ],
            ) {
                Ok(obj) => Ok(obj.downcast_unchecked()),
                Err(_) => Err(glib::BoolError("Failed to create application")),
            }
        }
    }
}

#[test]
fn test_arguments() {
    let app =
        SimpleApplication::new("org.gtk.TestApplication", gio::ApplicationFlags::empty()).unwrap();

    // FIXME: I have no clue what this does or why this is here
    app.set_inactivity_timeout(10000);

    assert!(app.run(&["--local".to_string()]) == EXIT_STATUS);
}
