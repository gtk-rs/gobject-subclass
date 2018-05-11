extern crate libc;
#[macro_use]
extern crate glib;
extern crate gio;

extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate gobject_subclass;

pub mod application;
