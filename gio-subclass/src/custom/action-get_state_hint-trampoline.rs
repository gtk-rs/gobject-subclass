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

    match imp.get_state_hint(&wrap){
        Some(t)  => {
            let ret = t.to_glib_full();
            gobject_ffi::g_object_set_qdata_full(gptr as *mut gobject_ffi::GObject,
                glib_ffi::g_quark_from_string("rs_action_state_hint".to_glib_none().0),
                ret as *mut c_void,
                None //TODO: how do we free the data here?
            );
            ret
        },
        None => ptr::null_mut()
    }
}
