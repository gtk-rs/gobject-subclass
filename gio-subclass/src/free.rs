use glib_ffi;
use glib;

/// Free a raw glib pointer
pub trait FreeGlibPtr {
    #[inline]
    unsafe extern "C" fn free(ptr: glib_ffi::gpointer){
        glib_ffi::g_free(ptr);
    }
}

impl FreeGlibPtr for String{
    #[inline]
    unsafe extern "C" fn free(ptr: glib_ffi::gpointer){
        glib_ffi::g_free(ptr);
    }
}

impl FreeGlibPtr for glib::VariantType{
    #[inline]
    unsafe extern "C" fn free(ptr: glib_ffi::gpointer){
        glib_ffi::g_variant_type_free(ptr as *mut glib_ffi::GVariantType);
    }
}
