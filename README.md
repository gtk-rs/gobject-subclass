# gobject-subclass [![crates.io](https://img.shields.io/crates/v/gobject-subclass.svg)](https://crates.io/crates/gobject-subclass) [![Build Status](https://travis-ci.org/gtk-rs/gobject-subclass.svg?branch=master)](https://travis-ci.org/gtk-rs/gobject-subclass)

# This crate is deprecated and all functionality from in here was merged into [glib-rs](https://github.com/gtk-rs/glib) with the 0.7 release

Infrastructure for writing [GObject](https://developer.gnome.org/gobject/stable/)
subclasses in the [Rust programming language](https://www.rust-lang.org/), and (in the future)
bindings for GObject and [GIO](https://developer.gnome.org/gio/stable/)
classes.

Example usage of this can be found here

 * [gst-plugin-rs](https://github.com/sdroege/gst-plugin-rs): GStreamer plugin
   writing infrastructure
 * [gtk-subclass](https://github.com/sdroege/gtk-subclass): GTK class bindings
 * [Listbox Model Data implementation](https://github.com/gtk-rs/examples/raw/master/src/bin/listbox_model.rs):
   This is a good, quick example of how to create a GObject conforming struct in Rust.
   It shows how to make a GObject data class for using with `gio::ListStore` which is then passed to `gtk::ListBox.bind_model()`

This is different to [gnome-class](https://gitlab.gnome.org/federico/gnome-class)
as it does not require usage of a C#-like DSL in a heavy procedural macro, but
instead works directly with Rust traits. Both are built on top of the
user-level GLib/GObject/GTK [gtk-rs](http://www.gtk-rs.org) bindings.

It will likely not support all features `gnome-class` will support. For
example it is not easily possible to create new GObject subclasses and expose
them to C with the whole instance/class struct and allow adding new virtual
methods. Subclassing and overriding of existing GObject virtual methods from C
however is perfectly possible.

## LICENSE

gobject-subclass and all crates contained in here that are not listed below are
licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

GLib/GObject itself is licensed under the Lesser General Public License version
2.1 or (at your option) any later version:
https://www.gnu.org/licenses/lgpl-2.1.html

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gobject-subclass by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
