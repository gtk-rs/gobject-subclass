// This file was generated by gir (https://github.com/gtk-rs/gir @ 1e22dcb+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 47c69e6)
// DO NOT EDIT

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib_ffi::{gboolean, gconstpointer, gpointer, GType};

use free::*;

#[cfg(any(feature = "v2_38", feature = "dox"))]
use Error;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;


pub trait ActionImpl: AnyImpl + 'static {

    fn activate(&self, action: &gio::Action, parameter: Option<&glib::Variant>);

    fn change_state(&self, action: &gio::Action, value: &glib::Variant);

    fn get_enabled(&self, action: &gio::Action) -> bool;

    fn get_name(&self, action: &gio::Action) -> Option<String>;

    fn get_parameter_type(&self, action: &gio::Action) -> Option<glib::VariantType>;

    fn get_state(&self, action: &gio::Action) -> Option<glib::Variant>;

    fn get_state_hint(&self, action: &gio::Action) -> Option<glib::Variant>;

    fn get_state_type(&self, action: &gio::Action) -> Option<glib::VariantType>;

}

any_impl!(ActionImpl);

pub trait ActionImplStatic<T: ObjectType>: 'static {
    fn get_impl<'a>(&self, imp: &'a T::ImplType) -> &'a ActionImpl;
}

struct ActionStatic<T: ObjectType>{
    imp_static: *const ActionImplStatic<T>
}

// FIXME: Boilerplate

unsafe extern "C" fn action_activate<T: ObjectType>
(gptr: *mut gio_ffi::GAction, parameter: *mut glib_ffi::GVariant)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.activate(&wrap, (if parameter.is_null() { None } else { Some(from_glib_none(parameter)) }).as_ref())
}

unsafe extern "C" fn action_change_state<T: ObjectType>
(gptr: *mut gio_ffi::GAction, value: *mut glib_ffi::GVariant)
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.change_state(&wrap, &from_glib_none(value))
}

unsafe extern "C" fn action_get_enabled<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> gboolean
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    imp.get_enabled(&wrap).to_glib()
}

unsafe extern "C" fn action_get_name<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> *const c_char
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);

    unsafe extern "C" fn destroy(p: glib_ffi::gpointer){
        glib_ffi::g_free(p);
    };

    match imp.get_name(&wrap)
    {
        Some(t) => {
            let ret = t/*Not checked*/.to_glib_none().0;
            gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
                glib_ffi::g_quark_from_string("rs_get_name".to_glib_none().0),
                ret as *mut c_void,
                Some(destroy)
            );
            ret
        },
        None => ptr::null()
    }
}

unsafe extern "C" fn action_get_parameter_type<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> *const glib_ffi::GVariantType
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);

    unsafe extern "C" fn destroy(p: glib_ffi::gpointer){
        glib_ffi::g_variant_type_free(p as *mut glib_ffi::GVariantType);
    };

    match imp.get_parameter_type(&wrap)
    {
        Some(t) => {
            let ret = t/*Not checked*/.to_glib_none().0;
            gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
                glib_ffi::g_quark_from_string("rs_get_parameter_type".to_glib_none().0),
                ret as *mut c_void,
                Some(destroy)
            );
            ret
        },
        None => ptr::null()
    }
}

unsafe extern "C" fn action_get_state<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> *mut glib_ffi::GVariant
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    match imp.get_state(&wrap){ Some(t)  => t.to_glib_full(), None => ptr::null_mut()}
}

unsafe extern "C" fn action_get_state_hint<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> *mut glib_ffi::GVariant
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);
    match imp.get_state_hint(&wrap){ Some(t)  => t.to_glib_full(), None => ptr::null_mut()}
}

unsafe extern "C" fn action_get_state_type<T: ObjectType>
(gptr: *mut gio_ffi::GAction) -> *const glib_ffi::GVariantType
{
    floating_reference_guard!(gptr);
    let klass = &**(gptr as *const *const ClassStruct<T>);
    let interface_static = klass.get_interface_static(gio_ffi::g_action_get_type())
                                     as *const ActionStatic<T>;
    let instance = &*(gptr as *const T::InstanceStructType);
    let imp = instance.get_impl();
    let imp = (*(*interface_static).imp_static).get_impl(imp);
    let wrap = from_glib_borrow(gptr);

    unsafe extern "C" fn destroy(p: glib_ffi::gpointer){
        glib_ffi::g_free(p);
    };

    match imp.get_state_type(&wrap)
    {
        Some(t) => {
            let ret = t/*Not checked*/.to_glib_none().0;
            gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
                glib_ffi::g_quark_from_string("rs_get_state_type".to_glib_none().0),
                ret as *mut c_void,
                Some(destroy)
            );
            ret
        },
        None => ptr::null()
    }
}

unsafe extern "C" fn action_init<T: ObjectType>(
    iface: glib_ffi::gpointer,
    iface_data: glib_ffi::gpointer
) {
    let action_iface = &mut *(iface as *mut gio_ffi::GActionInterface);
    let iface_type = (*(iface as *const gobject_ffi::GTypeInterface)).g_type;
    let type_ = (*(iface as *const gobject_ffi::GTypeInterface)).g_instance_type;
    let klass = &mut *(gobject_ffi::g_type_class_ref(type_) as *mut ClassStruct<T>);
    let interfaces_static = &mut *(klass.interfaces_static as *mut Vec<_>);
    interfaces_static.push((iface_type, iface_data));
    action_iface.activate = Some(action_activate::<T>);
    action_iface.change_state = Some(action_change_state::<T>);
    action_iface.get_enabled = Some(action_get_enabled::<T>);
    action_iface.get_name = Some(action_get_name::<T>);
    action_iface.get_parameter_type = Some(action_get_parameter_type::<T>);
    action_iface.get_state = Some(action_get_state::<T>);
    action_iface.get_state_hint = Some(action_get_state_hint::<T>);
    action_iface.get_state_type = Some(action_get_state_type::<T>);
}

pub fn register_action<T: ObjectType, I: ActionImplStatic<T>>(
    _: &TypeInitToken,
    type_: glib::Type,
    imp: &I,
) {

    unsafe {
        let imp = imp as &ActionImplStatic<T> as *const ActionImplStatic<T>;
        let interface_static = Box::new(ActionStatic {
            imp_static: imp,
        });
        let iface_info = gobject_ffi::GInterfaceInfo {
            interface_init: Some(action_init::<T>),
            interface_finalize: None,
            interface_data: Box::into_raw(interface_static) as glib_ffi::gpointer,
        };
        gobject_ffi::g_type_add_interface_static(
            type_.to_glib(),
            gio_ffi::g_action_get_type(),
            &iface_info,
        );
    }

}
