// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use glib_ffi;
use gobject_ffi;

use std::mem;
use std::ptr;

use glib;
use glib::translate::*;

use prelude::*;
use properties::*;
use types;

// TODO: This should be moved to glib-rs

/// Trait for implementors of `glib::Object` subclasses
///
/// This allows overriding the virtual methods of `glib::Object`
pub trait ObjectImpl: 'static {
    // TODO: Define a macro for this
    /// Storage for the type-specific data used during registration
    ///
    /// This is usually generated by the TODO macro.
    fn get_type_data(&self) -> ptr::NonNull<types::TypeData>;

    /// Property setter
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    fn set_property(&self, _obj: &glib::Object, _id: u32, _value: &glib::Value) {
        unimplemented!()
    }

    /// Property getter
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    fn get_property(&self, _obj: &glib::Object, _id: u32) -> Result<glib::Value, ()> {
        unimplemented!()
    }

    /// Constructed
    ///
    /// This is called once construction of the instance is finished.
    ///
    /// Should chain up to the parent class' implementation.
    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }

    /// Chain up to the parent class' implementation of `glib::Object::constructed()`
    ///
    /// Do not override this, it has no effect.
    fn parent_constructed(&self, obj: &glib::Object) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).constructed {
                func(obj.to_glib_none().0);
            }
        }
    }
}

/// Class struct for `glib::Object`
#[repr(C)]
pub struct ObjectClass(gobject_ffi::GObjectClass);

unsafe impl IsClassFor<glib::Object> for ObjectClass {}

/// Extension trait for `glib::Object`'s class struct
///
/// This contains various class methods and allows subclasses
/// to override the virtual methods
pub trait ObjectClassExt: Sized + 'static {
    /// Override the virtual methods
    fn override_vfuncs(&mut self)
    where
        Self: ClassStruct,
    {
        unsafe {
            let klass = &mut *(self as *const Self as *mut gobject_ffi::GObjectClass);
            klass.set_property = Some(set_property::<<Self as ClassStruct>::Type>);
            klass.get_property = Some(get_property::<<Self as ClassStruct>::Type>);
            klass.constructed = Some(constructed::<<Self as ClassStruct>::Type>);
        }
    }

    /// Install properties on the subclass
    ///
    /// This must be called after `override_vfuncs()` to work correctly.
    /// The index in the properties array is going to be the index passed to the
    /// property setters and getters.
    // TODO: Use a different Property struct
    //   struct Property {
    //     name: &'static str,
    //     pspec: fn () -> glib::ParamSpec,
    //   }
    fn install_properties(&mut self, properties: &[Property]) {
        if properties.is_empty() {
            return;
        }

        let mut pspecs = Vec::with_capacity(properties.len());

        pspecs.push(ptr::null_mut());

        for property in properties {
            pspecs.push(property.into());
        }

        unsafe {
            gobject_ffi::g_object_class_install_properties(
                self as *mut _ as *mut gobject_ffi::GObjectClass,
                pspecs.len() as u32,
                pspecs.as_mut_ptr(),
            );
        }
    }

    /// Add a new signal to the subclass
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    fn add_signal(&mut self, name: &str, arg_types: &[glib::Type], ret_type: glib::Type) {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }

    /// Add a new signal with accumulator to the subclass
    ///
    /// This can be emitted later by `glib::Object::emit` and external code
    /// can connect to the signal to get notified about emissions.
    ///
    /// The accumulator function is used for accumulating the return values of
    /// multiple signal handlers. The new value is passed as second argument and
    /// should be combined with the old value in the first argument. If no further
    /// signal handlers should be called, `false` should be returned.
    fn add_signal_with_accumulator<F>(
        &mut self,
        name: &str,
        arg_types: &[glib::Type],
        ret_type: glib::Type,
        accumulator: F,
    ) where
        F: Fn(&mut glib::Value, &glib::Value) -> bool + Send + Sync + 'static,
    {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();

        let accumulator: Box<
            Box<Fn(&mut glib::Value, &glib::Value) -> bool + Send + Sync + 'static>,
        > = Box::new(Box::new(accumulator));

        unsafe extern "C" fn accumulator_trampoline(
            _ihint: *mut gobject_ffi::GSignalInvocationHint,
            return_accu: *mut gobject_ffi::GValue,
            handler_return: *const gobject_ffi::GValue,
            data: glib_ffi::gpointer,
        ) -> glib_ffi::gboolean {
            let accumulator: &&(Fn(&mut glib::Value, &glib::Value) -> bool
                                    + Send
                                    + Sync
                                    + 'static) =
                &*(data as *const &(Fn(&mut glib::Value, &glib::Value) -> bool
                      + Send
                      + Sync
                      + 'static));
            accumulator(
                &mut *(return_accu as *mut glib::Value),
                &*(handler_return as *const glib::Value),
            )
            .to_glib()
        }

        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST,
                ptr::null_mut(),
                Some(accumulator_trampoline),
                Box::into_raw(accumulator) as glib_ffi::gpointer,
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }

    /// Add a new action signal with accumulator to the subclass
    ///
    /// Different to normal signals, action signals are supposed to be emitted
    /// by external code and will cause the provided handler to be called.
    ///
    /// It can be thought of as a dynamic function call.
    fn add_action_signal<F>(
        &mut self,
        name: &str,
        arg_types: &[glib::Type],
        ret_type: glib::Type,
        handler: F,
    ) where
        F: Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static,
    {
        let arg_types = arg_types.iter().map(|t| t.to_glib()).collect::<Vec<_>>();
        let handler = glib::Closure::new(handler);
        unsafe {
            gobject_ffi::g_signal_newv(
                name.to_glib_none().0,
                *(self as *mut _ as *mut glib_ffi::GType),
                gobject_ffi::G_SIGNAL_RUN_LAST | gobject_ffi::G_SIGNAL_ACTION,
                handler.to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
                ret_type.to_glib(),
                arg_types.len() as u32,
                arg_types.as_ptr() as *mut _,
            );
        }
    }
}

impl<T: IsAClass<ObjectClass> + 'static> ObjectClassExt for T {}

unsafe extern "C" fn get_property<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    match imp.get_property(&from_glib_borrow(obj), id - 1) {
        Ok(v) => {
            // Here we overwrite the value directly with ours
            // and forget ours because otherwise we would do
            // an additional copy of the value, which for
            // non-refcounted types involves a deep copy
            gobject_ffi::g_value_unset(value);
            ptr::write(value, ptr::read(v.to_glib_none().0));
            mem::forget(v);
        }
        Err(()) => eprintln!("Failed to get property"),
    }
}

unsafe extern "C" fn set_property<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    _pspec: *mut gobject_ffi::GParamSpec,
) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();
    imp.set_property(
        &from_glib_borrow(obj),
        id - 1,
        &*(value as *mut glib::Value),
    );
}

unsafe extern "C" fn constructed<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    floating_reference_guard!(obj);
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.get_impl();

    imp.constructed(&from_glib_borrow(obj));
}
