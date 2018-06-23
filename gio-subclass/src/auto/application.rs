// This file was generated by gir (https://github.com/gtk-rs/gir @ 7332f9f)
// from gir-files (https://github.com/gtk-rs/gir-files @ 47c69e6)
// DO NOT EDIT

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib_ffi::{gboolean, gconstpointer, gpointer, GType};

use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;
use action_group as ;

use std::fmt;
use std::ffi::OsString;
use std::convert;
use std::ops::Deref;

pub struct ArgumentList {
    pub(crate) ptr: *mut *mut *mut c_char,
    items: Vec<OsString>,
}

impl ArgumentList {
    pub(crate) fn new(arguments: *mut *mut *mut c_char) -> Self {
        Self {
            ptr: arguments,
            items: unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(arguments)) },
        }
    }

    pub(crate) fn refresh(&mut self) {
        self.items = unsafe { FromGlibPtrContainer::from_glib_none(ptr::read(self.ptr)) };
    }

    // remove the item at index `idx` and shift the raw array
    pub fn remove(&mut self, idx: usize) {
        unsafe {
            let n_args = glib_ffi::g_strv_length(*self.ptr);
            assert!((n_args as usize) == self.items.len());
            assert!((idx as u32) < n_args);

            self.items.remove(idx);

            glib_ffi::g_free(((*self.ptr).offset(idx as isize)) as *mut c_void);

            for i in (idx as u32)..n_args - 1 {
                ptr::write(
                    (*self.ptr).offset(i as isize),
                    *(*self.ptr).offset((i + 1) as isize),
                )
            }
            ptr::write((*self.ptr).offset((n_args - 1) as isize), ptr::null_mut());
        }
    }
}

impl Deref for ArgumentList {
    type Target = [OsString];

    fn deref(&self) -> &Self::Target {
        self.items.as_slice()
    }
}

impl fmt::Debug for ArgumentList {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.items.fmt(formatter)
    }
}

impl convert::Into<Vec<OsString>> for ArgumentList {
    fn into(self) -> Vec<OsString> {
        self.items.clone()
    }
}

pub trait ApplicationImpl<T: ApplicationBase>: ObjectImpl<T> + AnyImpl + 'static {

    fn activate(&self, application: &T){
        application.parent_activate()
    }

    fn after_emit(&self, application: &T, platform_data: &glib::Variant){
        application.parent_after_emit(platform_data)
    }

    fn before_emit(&self, application: &T, platform_data: &glib::Variant){
        application.parent_before_emit(platform_data)
    }

    fn command_line(&self, application: &T, command_line: /*Ignored*/&gio::ApplicationCommandLine) -> i32{
        application.parent_command_line(command_line)
    }

    fn local_command_line(&self, application: &T, arguments: &mut ArgumentList) -> Option<i32> {
        application.parent_local_command_line(arguments)
    }

    fn open(&self, application: &T, files: /*Ignored*/&[gio::File], hint: &str){
        application.parent_open(files, hint)
    }

    fn quit_mainloop(&self, application: &T){
        application.parent_quit_mainloop()
    }

    fn run_mainloop(&self, application: &T){
        application.parent_run_mainloop()
    }

    fn shutdown(&self, application: &T){
        application.parent_shutdown()
    }

    fn startup(&self, application: &T){
        application.parent_startup()
    }

}

pub trait ApplicationImplExt<T> {}
impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gio::Application>> ApplicationImplExt<T> for S {}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: ObjectType + glib::IsA<gio::Application>{

    fn parent_activate(&self){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .activate
            .map(|f|{ f(self.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_after_emit(&self, platform_data: &glib::Variant){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .after_emit
            .map(|f|{ f(self.to_glib_none().0,platform_data.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_before_emit(&self, platform_data: &glib::Variant){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .before_emit
            .map(|f|{ f(self.to_glib_none().0,platform_data.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_command_line(&self, cmd_line: &gio::ApplicationCommandLine) -> i32 {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .command_line
                .map(|f| f(self.to_glib_none().0, cmd_line.to_glib_none().0))
                .unwrap_or(0)
        }
    }

    fn parent_local_command_line(&self, arguments: &mut ArgumentList) -> Option<i32> {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            let mut exit_status = 0;
            let success = (*parent_klass)
                .local_command_line
                .map(|f| {
                    let ret = f(self.to_glib_none().0, arguments.ptr, &mut exit_status);
                    arguments.refresh();
                    ret
                })
                .unwrap_or(glib_ffi::GFALSE);

            match success {
                glib_ffi::GTRUE => Some(exit_status),
                _ => None,
            }
        }
    }

    fn parent_open(&self, files: /*Ignored*/&[gio::File], hint: &str){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .open
            .map(|f|{ let n_files = files.len() as i32; f(self.to_glib_none().0,files.to_glib_none().0,n_files,hint.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_quit_mainloop(&self){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .quit_mainloop
            .map(|f|{ f(self.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_run_mainloop(&self){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .run_mainloop
            .map(|f|{ f(self.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_shutdown(&self){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .shutdown
            .map(|f|{ f(self.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_startup(&self){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
            .startup
            .map(|f|{ f(self.to_glib_none().0); })
            .unwrap_or(())
        }
    }

}

pub unsafe trait ApplicationClassExt<T: ApplicationBase>
where
    T::ImplType: ApplicationImpl<T>{

    fn override_vfuncs(&mut self, _: &ClassInitToken){
        unsafe {
            let klass = &mut *(self as *const Self as *mut gio_ffi::GApplicationClass);
            klass.activate = Some(application_activate::<T>);
            klass.after_emit = Some(application_after_emit::<T>);
            klass.before_emit = Some(application_before_emit::<T>);
            klass.command_line = Some(application_command_line::<T>);
            klass.local_command_line = Some(application_local_command_line::<T>);
            klass.open = Some(application_open::<T>);
            klass.quit_mainloop = Some(application_quit_mainloop::<T>);
            klass.run_mainloop = Some(application_run_mainloop::<T>);
            klass.shutdown = Some(application_shutdown::<T>);
            klass.startup = Some(application_startup::<T>);
        }
    }

}

glib_wrapper! {

    pub struct Application(Object<InstanceStruct<Application>>):[
         gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap,]    ;
    match fn { 
         get_type => || get_type::<Application>(),
     }

}

unsafe impl<T: ObjectType + glib::IsA<gio::Application>> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

unsafe impl ObjectClassExt<Application> for ApplicationClass {}
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_gio_application_impl(
    ($name:ident) => {
        box_object_impl!($name);
        impl<T: $crate::application::ApplicationBase> $crate::application::ApplicationImpl<T> for Box<$name<T>>{

            fn activate(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.activate(application)
            }

            fn after_emit(&self, application: &T, platform_data: &glib::Variant){
                let imp: &$name<T> = self.as_ref();
                imp.after_emit(application, platform_data)
            }

            fn before_emit(&self, application: &T, platform_data: &glib::Variant){
                let imp: &$name<T> = self.as_ref();
                imp.before_emit(application, platform_data)
            }

            fn command_line(&self, application: &T, command_line: /*Ignored*/&gio::ApplicationCommandLine) -> i32{
                let imp: &$name<T> = self.as_ref();
                imp.command_line(application, command_line)
            }

            fn local_command_line(&self, application: &T, arguments: &mut ArgumentList) -> Option<i32>{
                let imp: &$name<T> = self.as_ref();
                imp.local_command_line(application, arguments)
            }

            fn open(&self, application: &T, files: /*Ignored*/&[gio::File], hint: &str){
                let imp: &$name<T> = self.as_ref();
                imp.open(application, files, hint)
            }

            fn quit_mainloop(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.quit_mainloop(application)
            }

            fn run_mainloop(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.run_mainloop(application)
            }

            fn shutdown(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.shutdown(application)
            }

            fn startup(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.startup(application)
            }
        }
    }
);

box_gio_application_impl!(ApplicationImpl);

impl ObjectType for Application{
    const NAME: &'static str = "RsGioApplication";
    type ParentType = gio::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;
    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
    }
    object_type_fns!();
}

unsafe extern "C" fn application_activate<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.activate(&wrap);
}

unsafe extern "C" fn application_after_emit<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication, platform_data: *mut glib_ffi::GVariant)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.after_emit(&wrap, &from_glib_none(platform_data));
}

unsafe extern "C" fn application_before_emit<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication, platform_data: *mut glib_ffi::GVariant)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.before_emit(&wrap, &from_glib_none(platform_data));
}

unsafe extern "C" fn application_command_line<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication, command_line: *mut gio_ffi::GApplicationCommandLine) -> c_int
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    let rs_ret = imp.command_line(&wrap, &from_glib_none(command_line));
    rs_ret
}
unsafe extern "C" fn application_local_command_line<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    arguments: *mut *mut *mut c_char,
    exit_status: *mut c_int,
) -> glib_ffi::gboolean
where
    T::ImplType: ApplicationImpl<T>,
{
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    let mut args = ArgumentList::new(arguments);

    match imp.local_command_line(&wrap, &mut args) {
        Some(ret) => {
            ptr::write(exit_status, ret);
            glib_ffi::GTRUE
        }
        None => glib_ffi::GFALSE,
    }
}
unsafe extern "C" fn application_open<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    files: *mut *mut gio_ffi::GFile,
    num_files: c_int,
    hint: *const c_char,
) where
    T::ImplType: ApplicationImpl<T>,
{
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    let files_r: Vec<gio::File> = FromGlibContainer::from_glib_none_num(files, num_files as usize);
    let hint_r: String = from_glib_none(hint);
    imp.open(&wrap, &files_r.as_slice(), &hint_r.as_str())
}

unsafe extern "C" fn application_quit_mainloop<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.quit_mainloop(&wrap);
}

unsafe extern "C" fn application_run_mainloop<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.run_mainloop(&wrap);
}

unsafe extern "C" fn application_shutdown<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.shutdown(&wrap);
}

unsafe extern "C" fn application_startup<T: ApplicationBase>
(gptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>
{
    floating_reference_guard!(gptr);
    let application = &*(gptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(gptr as *mut T::InstanceStructType);
    let imp = application.get_impl();
    imp.startup(&wrap);
}
