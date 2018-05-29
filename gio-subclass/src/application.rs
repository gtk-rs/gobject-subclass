use gio;
use gio_ffi;
use glib;
use glib::translate::*;
use glib::IsA;
use glib_ffi;
use gobject_ffi;
use libc;
use std::convert;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::ffi::OsString;

use gobject_subclass::anyimpl::*;
use gobject_subclass::object::*;

pub struct ArgumentList {
    pub(crate) ptr: *mut *mut *mut libc::c_char,
    items: Vec<OsString>,
}

impl ArgumentList {
    pub(crate) fn new(arguments: *mut *mut *mut libc::c_char) -> Self {
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

            glib_ffi::g_free(((*self.ptr).offset(idx as isize)) as *mut libc::c_void);

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
    fn startup(&self, application: &T) {
        application.parent_startup();
    }

    fn activate(&self, application: &T) {
        application.parent_activate();
    }

    fn open(&self, application: &T, files: &[gio::File], hint: &str) {
        application.parent_open(files, hint)
    }

    fn command_line(&self, application: &T, cmd_line: &gio::ApplicationCommandLine) -> i32 {
        application.parent_command_line(cmd_line)
    }

    fn local_command_line(&self, application: &T, arguments: &mut ArgumentList) -> Option<i32> {
        application.parent_local_command_line(arguments)
    }

    fn before_emit(&self, application: &T, platform_data: &glib::Variant) {
        application.parent_before_emit(platform_data)
    }

    fn after_emit(&self, application: &T, platform_data: &glib::Variant) {
        application.parent_after_emit(platform_data)
    }

    // fn add_platform_data(&self, application: &T, builder: &glib::VariantBuilder){
    //     application.parent_add_platform_data(builder)
    // }

    fn run_mainloop(&self, application: &T) {
        application.parent_run_mainloop();
    }

    fn quit_mainloop(&self, application: &T) {
        application.parent_quit_mainloop();
    }

    fn shutdown(&self, application: &T) {
        application.parent_shutdown();
    }

    // fn handle_local_options(&self, application: &T, options: &glib::VariantDict) {
    //     application.handle_local_options(options);
    // }
    //
}

pub trait ApplicationImplExt<T> {}

impl<S: ApplicationImpl<T>, T: ObjectType + glib::IsA<gio::Application>> ApplicationImplExt<T>
    for S
{
}

any_impl!(ApplicationBase, ApplicationImpl);

pub unsafe trait ApplicationBase: IsA<gio::Application> + ObjectType {
    fn parent_startup(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .startup
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_activate(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .activate
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_open(&self, files: &[gio::File], hint: &str) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .open
                .map(|f| {
                    let n_files = files.len() as i32;
                    f(
                        self.to_glib_none().0,
                        files.to_glib_none().0,
                        n_files,
                        hint.to_glib_none().0,
                    )
                })
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

    fn parent_before_emit(&self, platform_data: &glib::Variant) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .before_emit
                .map(|f| f(self.to_glib_none().0, platform_data.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_after_emit(&self, platform_data: &glib::Variant) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .after_emit
                .map(|f| f(self.to_glib_none().0, platform_data.to_glib_none().0))
                .unwrap_or(())
        }
    }

    // fn parent_add_platform_data(&self, builder: &glib::VariantBuilder) {
    //     unsafe {
    //         let klass = self.get_class();
    //         let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
    //         (*parent_klass)
    //             .add_platform_data
    //             .map(|f| f(self.to_glib_none().0, builder.to_glib_none().0))
    //             .unwrap_or(())
    //     }
    // }

    fn parent_run_mainloop(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .run_mainloop
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_quit_mainloop(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .quit_mainloop
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_shutdown(&self) {
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
            (*parent_klass)
                .shutdown
                .map(|f| f(self.to_glib_none().0))
                .unwrap_or(())
        }
    }

    //...

    // fn parent_handle_local_options(&self, options: &glib::VariantDict) {
    //     unsafe {
    //         let klass = self.get_class();
    //         let parent_klass = (*klass).get_parent_class() as *const gio_ffi::GApplicationClass;
    //         (*parent_klass)
    //             .handle_local_options
    //             .map(|f| f(self.to_glib_none().0, options.to_glib_none().0))
    //             .unwrap_or(())
    //     }
    // }
}

pub unsafe trait ApplicationClassExt<T: ApplicationBase>
where
    T::ImplType: ApplicationImpl<T>,
{
    fn override_vfuncs(&mut self, _: &ClassInitToken) {
        unsafe {
            let klass = &mut *(self as *const Self as *mut gio_ffi::GApplicationClass);
            klass.startup = Some(application_startup::<T>);
            klass.activate = Some(application_activate::<T>);
            klass.open = Some(application_open::<T>);
            klass.command_line = Some(application_command_line::<T>);
            klass.local_command_line = Some(application_local_command_line::<T>);
            klass.before_emit = Some(application_before_emit::<T>);
            klass.after_emit = Some(application_after_emit::<T>);
            // klass.add_platform_data = Some(application_add_platform_data::<T>);
            klass.quit_mainloop = Some(application_quit_mainloop::<T>);
            klass.run_mainloop = Some(application_run_mainloop::<T>);
            klass.shutdown = Some(application_shutdown::<T>);

            //...
            // klass.handle_local_options = Some(application_handle_local_options::<T>);
        }
    }
}

glib_wrapper! {
    pub struct Application(Object<InstanceStruct<Application>>):
        [gio::Application => gio_ffi::GApplication,
         gio::ActionGroup => gio_ffi::GActionGroup,
         gio::ActionMap => gio_ffi::GActionMap];

    match fn {
        get_type => || get_type::<Application>(),
    }
}

unsafe impl<T: IsA<gio::Application> + ObjectType> ApplicationBase for T {}

pub type ApplicationClass = ClassStruct<Application>;

// FIXME: Boilerplate
unsafe impl ApplicationClassExt<Application> for ApplicationClass {}
unsafe impl ObjectClassExt<Application> for ApplicationClass {}

#[macro_export]
macro_rules! box_gapplication_impl(
    ($name:ident) => {
        box_object_impl!($name);

        impl<T: $crate::application::ApplicationBase>  $crate::application::ApplicationImpl<T> for Box<$name<T>>
        {
            fn startup(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.startup(application)
            }

            fn activate(&self, application: &T){
                let imp: &$name<T> = self.as_ref();
                imp.activate(application)
            }

            fn open(&self, application: &T, files: &[gio::File], hint: &str){
                let imp: &$name<T> = self.as_ref();
                imp.open(application, files, hint)
            }

            fn command_line(&self, application: &T, cmd_line: &gio::ApplicationCommandLine) -> i32{
                let imp: &$name<T> = self.as_ref();
                imp.command_line(application, cmd_line)
            }

            fn local_command_line(&self, application: &T, arguments: &mut ArgumentList) -> Option<i32>{
                let imp: &$name<T> = self.as_ref();
                imp.local_command_line(application, arguments)
            }

            fn before_emit(&self, application: &T, platform_data: &glib::Variant){
                let imp: &$name<T> = self.as_ref();
                imp.before_emit(application, platform_data)
            }

            fn after_emit(&self, application: &T, platform_data: &glib::Variant){
                let imp: &$name<T> = self.as_ref();
                imp.after_emit(application, platform_data)
            }

            // no translation for GVariantBuilder
            // fn add_platform_data(&self, application: &T, builder: &glib::VariantBuilder){
            //     let imp: &$name<T> = self.as_ref();
            //     imp.add_platform_data(application, builder)
            // }

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

            // no translation for GVariantDict
            // fn handle_local_options(&self, application: &T, options: &glib::VariantDict){
            //     let imp: &$name<T> = self.as_ref();
            //     imp.handle_local_options(application, options)
            // }

        }
    };
);

box_gapplication_impl!(ApplicationImpl);

impl ObjectType for Application {
    const NAME: &'static str = "RsGApplication";
    type ParentType = gio::Application;
    type ImplType = Box<ApplicationImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;

    fn class_init(token: &ClassInitToken, klass: &mut ApplicationClass) {
        ObjectClassExt::override_vfuncs(klass, token);
        ApplicationClassExt::override_vfuncs(klass, token);
    }

    object_type_fns!();
}

unsafe extern "C" fn application_startup<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.startup(&wrap)
}

unsafe extern "C" fn application_activate<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.activate(&wrap)
}

unsafe extern "C" fn application_open<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    files: *mut *mut gio_ffi::GFile,
    num_files: libc::c_int,
    hint: *const libc::c_char,
) where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    let files_r: Vec<gio::File> = FromGlibContainer::from_glib_none_num(files, num_files as usize);
    let hint_r: String = from_glib_none(hint);
    imp.open(&wrap, &files_r.as_slice(), &hint_r.as_str())
}

unsafe extern "C" fn application_command_line<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    cmd_line: *mut gio_ffi::GApplicationCommandLine,
) -> libc::c_int
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.command_line(&wrap, &from_glib_borrow(cmd_line))
}

unsafe extern "C" fn application_local_command_line<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    arguments: *mut *mut *mut libc::c_char,
    exit_status: *mut libc::c_int,
) -> glib_ffi::gboolean
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
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

unsafe extern "C" fn application_before_emit<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    platform_data: *mut glib_ffi::GVariant,
) where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.before_emit(&wrap, &from_glib_borrow(platform_data))
}

unsafe extern "C" fn application_after_emit<T: ApplicationBase>(
    ptr: *mut gio_ffi::GApplication,
    platform_data: *mut glib_ffi::GVariant,
) where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.after_emit(&wrap, &from_glib_borrow(platform_data))
}

// No translation for GVariantBuilder
// unsafe extern "C" fn application_add_platform_data<T: ApplicationBase>(
//     ptr: *mut gio_ffi::GApplication,
//     builder: *mut glib_ffi::GVariantBuilder,
// )
// where
//     T::ImplType: ApplicationImpl<T>,
// {
//     callback_guard!();
//     floating_reference_guard!(ptr);
//     let application = &*(ptr as *mut T::InstanceStructType);
//     let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
//     let imp = application.get_impl();
//
//     imp.add_platform_data(&wrap, &from_glib_borrow(builder))
// }

unsafe extern "C" fn application_quit_mainloop<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.quit_mainloop(&wrap)
}

unsafe extern "C" fn application_run_mainloop<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.run_mainloop(&wrap)
}

unsafe extern "C" fn application_shutdown<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication)
where
    T::ImplType: ApplicationImpl<T>,
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let application = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = application.get_impl();

    imp.shutdown(&wrap)
}

// unsafe extern "C" fn application_dbus_register<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication,
//     connection: *mut gio_ffi::GDBusConnection,
//     object_path: *const libc::c_char,
//     error: *mut *mut glib_ffi::GError
// ) -> glib_ffi::gboolean
// where
//     T::ImplType: ApplicationImpl<T>,
// {
//     callback_guard!();
//     floating_reference_guard!(ptr);
//     let application = &*(ptr as *mut T::InstanceStructType);
//     let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
//     let imp = application.get_impl();
//
//     imp.dbus_register(&wrap, )
// }

// unsafe extern "C" fn application_dbus_unregister<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication,
//     connection: *mut gio_ffi::GDBusConnection,
//     object_path: *const libc::c_char
// ) -> glib_ffi::gboolean
// where
//     T::ImplType: ApplicationImpl<T>,
// {
//     callback_guard!();
//     floating_reference_guard!(ptr);
//     let application = &*(ptr as *mut T::InstanceStructType);
//     let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
//     let imp = application.get_impl();
//
//     imp.dbus_unregister(&wrap, &from_glib_borrow(connection), &from_glib_borrow(object_path))
// }

// unsafe extern "C" fn application_handle_local_options<T: ApplicationBase>(ptr: *mut gio_ffi::GApplication,
//     options: *mut glib_ffi::GVariantDict
// ) -> libc::c_int
// where
//     T::ImplType: ApplicationImpl<T>,
// {
//     callback_guard!();
//     floating_reference_guard!(ptr);
//     let application = &*(ptr as *mut T::InstanceStructType);
//     let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
//     let imp = application.get_impl();
//
//     imp.handle_local_options(&wrap,  &from_glib_borrow(options))
// }

//  pub padding: [gpointer; 8],
