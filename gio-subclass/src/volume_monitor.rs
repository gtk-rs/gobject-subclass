// This file was generated by gir (https://github.com/gtk-rs/gir @ a3d21e2+)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8+)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

use Drive;
use Mount;
use Volume;
use ffi;
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


pub trait VolumeMonitorImpl<T: VolumeMonitorBase>: ObjectImpl<T> + AnyImpl + 'static {

    fn drive_changed(&self, volume_monitor: &T, drive: &Drive){
        volume_monitor.parent_drive_changed(drive)
    }

    fn drive_connected(&self, volume_monitor: &T, drive: &Drive){
        volume_monitor.parent_drive_connected(drive)
    }

    fn drive_disconnected(&self, volume_monitor: &T, drive: &Drive){
        volume_monitor.parent_drive_disconnected(drive)
    }

    fn drive_eject_button(&self, volume_monitor: &T, drive: &Drive){
        volume_monitor.parent_drive_eject_button(drive)
    }

    fn drive_stop_button(&self, volume_monitor: &T, drive: &Drive){
        volume_monitor.parent_drive_stop_button(drive)
    }

    fn get_connected_drives(&self, volume_monitor: &T) -> Vec<Drive>{
        volume_monitor.parent_get_connected_drives()
    }

    fn get_mount_for_uuid(&self, volume_monitor: &T, uuid: &str) -> Option<Mount>{
        volume_monitor.parent_get_mount_for_uuid(uuid)
    }

    fn get_mounts(&self, volume_monitor: &T) -> Vec<Mount>{
        volume_monitor.parent_get_mounts()
    }

    fn get_volume_for_uuid(&self, volume_monitor: &T, uuid: &str) -> Option<Volume>{
        volume_monitor.parent_get_volume_for_uuid(uuid)
    }

    fn get_volumes(&self, volume_monitor: &T) -> Vec<Volume>{
        volume_monitor.parent_get_volumes()
    }

    fn mount_added(&self, volume_monitor: &T, mount: &Mount){
        volume_monitor.parent_mount_added(mount)
    }

    fn mount_changed(&self, volume_monitor: &T, mount: &Mount){
        volume_monitor.parent_mount_changed(mount)
    }

    fn mount_pre_unmount(&self, volume_monitor: &T, mount: &Mount){
        volume_monitor.parent_mount_pre_unmount(mount)
    }

    fn mount_removed(&self, volume_monitor: &T, mount: &Mount){
        volume_monitor.parent_mount_removed(mount)
    }

    fn volume_added(&self, volume_monitor: &T, volume: &Volume){
        volume_monitor.parent_volume_added(volume)
    }

    fn volume_changed(&self, volume_monitor: &T, volume: &Volume){
        volume_monitor.parent_volume_changed(volume)
    }

    fn volume_removed(&self, volume_monitor: &T, volume: &Volume){
        volume_monitor.parent_volume_removed(volume)
    }

}

pub trait VolumeMonitorImplExt<T> {}

}
impl<S: VolumeMonitorImpl<T>, T: ObjectType >> VolumeMonitorImplExt<T> for S {}

any_impl!(VolumeMonitorBase, VolumeMonitorImpl);

pub unsafe trait VolumeMonitorBase: ObjectType {

    fn parent_drive_changed(&self, drive: &Drive){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .drive_changed
            .map(|f|{ f(self.to_glib_none().0,drive.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_drive_connected(&self, drive: &Drive){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .drive_connected
            .map(|f|{ f(self.to_glib_none().0,drive.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_drive_disconnected(&self, drive: &Drive){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .drive_disconnected
            .map(|f|{ f(self.to_glib_none().0,drive.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_drive_eject_button(&self, drive: &Drive){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .drive_eject_button
            .map(|f|{ f(self.to_glib_none().0,drive.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_drive_stop_button(&self, drive: &Drive){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .drive_stop_button
            .map(|f|{ f(self.to_glib_none().0,drive.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_get_connected_drives(&self) -> Vec<Drive>{
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .get_connected_drives
            .map(|f|{ FromGlibPtrContainer::from_glib_full(f(self.to_glib_none().0)) })
            .unwrap_or(())
        }
    }

    fn parent_get_mount_for_uuid(&self, uuid: &str) -> Option<Mount>{
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .get_mount_for_uuid
            .map(|f|{ from_glib_full(f(self.to_glib_none().0,uuid.to_glib_none().0)) })
            .unwrap_or(())
        }
    }

    fn parent_get_mounts(&self) -> Vec<Mount>{
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .get_mounts
            .map(|f|{ FromGlibPtrContainer::from_glib_full(f(self.to_glib_none().0)) })
            .unwrap_or(())
        }
    }

    fn parent_get_volume_for_uuid(&self, uuid: &str) -> Option<Volume>{
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .get_volume_for_uuid
            .map(|f|{ from_glib_full(f(self.to_glib_none().0,uuid.to_glib_none().0)) })
            .unwrap_or(())
        }
    }

    fn parent_get_volumes(&self) -> Vec<Volume>{
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .get_volumes
            .map(|f|{ FromGlibPtrContainer::from_glib_full(f(self.to_glib_none().0)) })
            .unwrap_or(())
        }
    }

    fn parent_mount_added(&self, mount: &Mount){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .mount_added
            .map(|f|{ f(self.to_glib_none().0,mount.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_mount_changed(&self, mount: &Mount){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .mount_changed
            .map(|f|{ f(self.to_glib_none().0,mount.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_mount_pre_unmount(&self, mount: &Mount){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .mount_pre_unmount
            .map(|f|{ f(self.to_glib_none().0,mount.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_mount_removed(&self, mount: &Mount){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .mount_removed
            .map(|f|{ f(self.to_glib_none().0,mount.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_volume_added(&self, volume: &Volume){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .volume_added
            .map(|f|{ f(self.to_glib_none().0,volume.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_volume_changed(&self, volume: &Volume){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .volume_changed
            .map(|f|{ f(self.to_glib_none().0,volume.to_glib_none().0); })
            .unwrap_or(())
        }
    }

    fn parent_volume_removed(&self, volume: &Volume){
        unsafe {
            let klass = self.get_class();
            let parent_klass = (*klass).get_parent_class() as *const ffi::GVolumeMonitorClass;
            (*parent_klass)
            .volume_removed
            .map(|f|{ f(self.to_glib_none().0,volume.to_glib_none().0); })
            .unwrap_or(())
        }
    }

}

pub unsafe trait VolumeMonitorClassExt<T: VolumeMonitorBase>
where
    T::ImplType: VolumeMonitorImpl<T>{

    fn override_vfuncs(&mut self, _: &ClassInitToken){
        unsafe {
            let klass = &mut *(self as *const Self as *mut ffi::GVolumeMonitorClass);
            klass.drive_changed = Some(volumemonitor_drive_changed::<T>);
            klass.drive_connected = Some(volumemonitor_drive_connected::<T>);
            klass.drive_disconnected = Some(volumemonitor_drive_disconnected::<T>);
            klass.drive_eject_button = Some(volumemonitor_drive_eject_button::<T>);
            klass.drive_stop_button = Some(volumemonitor_drive_stop_button::<T>);
            klass.get_connected_drives = Some(volumemonitor_get_connected_drives::<T>);
            klass.get_mount_for_uuid = Some(volumemonitor_get_mount_for_uuid::<T>);
            klass.get_mounts = Some(volumemonitor_get_mounts::<T>);
            klass.get_volume_for_uuid = Some(volumemonitor_get_volume_for_uuid::<T>);
            klass.get_volumes = Some(volumemonitor_get_volumes::<T>);
            klass.mount_added = Some(volumemonitor_mount_added::<T>);
            klass.mount_changed = Some(volumemonitor_mount_changed::<T>);
            klass.mount_pre_unmount = Some(volumemonitor_mount_pre_unmount::<T>);
            klass.mount_removed = Some(volumemonitor_mount_removed::<T>);
            klass.volume_added = Some(volumemonitor_volume_added::<T>);
            klass.volume_changed = Some(volumemonitor_volume_changed::<T>);
            klass.volume_removed = Some(volumemonitor_volume_removed::<T>);
        }
    }

}

glib_wrapper! {

    pub struct VolumeMonitor(Object<InstanceStruct<VolumeMonitor>>)    ;
    match fn { 
         get_type => || get_type::<VolumeMonitor>(),
     }

}

unsafe impl<T: ObjectType > VolumeMonitorBase for T {}

pub type VolumeMonitorClass = ClassStruct<VolumeMonitor>;

// FIXME: Boilerplate

// FIXME: Boilerplate

#[macro_export]
macro_rules! box_volumemonitor_impl(
    ($name:ident) => {
        box_object_impl!($name);
        impl<T: $crate::volume_monitor::VolumeMonitorBase> $crate::volume_monitor::VolumeMonitorImpl<T> for Box<$name<T>>{

            fn drive_changed(&self, volume_monitor: &T, drive: &Drive){
                let imp: &$name<T> = self.as_ref();
                imp.drive_changed(drive)
            }

            fn drive_connected(&self, volume_monitor: &T, drive: &Drive){
                let imp: &$name<T> = self.as_ref();
                imp.drive_connected(drive)
            }

            fn drive_disconnected(&self, volume_monitor: &T, drive: &Drive){
                let imp: &$name<T> = self.as_ref();
                imp.drive_disconnected(drive)
            }

            fn drive_eject_button(&self, volume_monitor: &T, drive: &Drive){
                let imp: &$name<T> = self.as_ref();
                imp.drive_eject_button(drive)
            }

            fn drive_stop_button(&self, volume_monitor: &T, drive: &Drive){
                let imp: &$name<T> = self.as_ref();
                imp.drive_stop_button(drive)
            }

            fn get_connected_drives(&self, volume_monitor: &T) -> Vec<Drive>{
                let imp: &$name<T> = self.as_ref();
                imp.get_connected_drives()
            }

            fn get_mount_for_uuid(&self, volume_monitor: &T, uuid: &str) -> Option<Mount>{
                let imp: &$name<T> = self.as_ref();
                imp.get_mount_for_uuid(uuid)
            }

            fn get_mounts(&self, volume_monitor: &T) -> Vec<Mount>{
                let imp: &$name<T> = self.as_ref();
                imp.get_mounts()
            }

            fn get_volume_for_uuid(&self, volume_monitor: &T, uuid: &str) -> Option<Volume>{
                let imp: &$name<T> = self.as_ref();
                imp.get_volume_for_uuid(uuid)
            }

            fn get_volumes(&self, volume_monitor: &T) -> Vec<Volume>{
                let imp: &$name<T> = self.as_ref();
                imp.get_volumes()
            }

            fn mount_added(&self, volume_monitor: &T, mount: &Mount){
                let imp: &$name<T> = self.as_ref();
                imp.mount_added(mount)
            }

            fn mount_changed(&self, volume_monitor: &T, mount: &Mount){
                let imp: &$name<T> = self.as_ref();
                imp.mount_changed(mount)
            }

            fn mount_pre_unmount(&self, volume_monitor: &T, mount: &Mount){
                let imp: &$name<T> = self.as_ref();
                imp.mount_pre_unmount(mount)
            }

            fn mount_removed(&self, volume_monitor: &T, mount: &Mount){
                let imp: &$name<T> = self.as_ref();
                imp.mount_removed(mount)
            }

            fn volume_added(&self, volume_monitor: &T, volume: &Volume){
                let imp: &$name<T> = self.as_ref();
                imp.volume_added(volume)
            }

            fn volume_changed(&self, volume_monitor: &T, volume: &Volume){
                let imp: &$name<T> = self.as_ref();
                imp.volume_changed(volume)
            }

            fn volume_removed(&self, volume_monitor: &T, volume: &Volume){
                let imp: &$name<T> = self.as_ref();
                imp.volume_removed(volume)
            }
        }
    }
);

impl ObjectType for VolumeMonitor{
    const NAME: &'static str = "RsGio.VolumeMonitor";
    type ImplType = Box<VolumeMonitorImpl<Self>>;
    type InstanceStructType = InstanceStruct<Self>;
    fn class_init(token: &ClassInitToken, klass: &mut VolumeMonitorClass) {
        ObjectClassExt::override_vfuncs(klass, token);
    }
    object_type_fns!();
}

unsafe extern "C" fn volumemonitor_drive_changed<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, drive: *mut GDrive)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.drive_changed()
}

unsafe extern "C" fn volumemonitor_drive_connected<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, drive: *mut GDrive)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.drive_connected()
}

unsafe extern "C" fn volumemonitor_drive_disconnected<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, drive: *mut GDrive)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.drive_disconnected()
}

unsafe extern "C" fn volumemonitor_drive_eject_button<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, drive: *mut GDrive)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.drive_eject_button()
}

unsafe extern "C" fn volumemonitor_drive_stop_button<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, drive: *mut GDrive)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.drive_stop_button()
}

unsafe extern "C" fn volumemonitor_get_connected_drives<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor) -> *mut glib::GList
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.get_connected_drives()
}

unsafe extern "C" fn volumemonitor_get_mount_for_uuid<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, uuid: *const c_char) -> *mut GMount
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.get_mount_for_uuid()
}

unsafe extern "C" fn volumemonitor_get_mounts<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor) -> *mut glib::GList
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.get_mounts()
}

unsafe extern "C" fn volumemonitor_get_volume_for_uuid<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, uuid: *const c_char) -> *mut GVolume
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.get_volume_for_uuid()
}

unsafe extern "C" fn volumemonitor_get_volumes<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor) -> *mut glib::GList
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.get_volumes()
}

unsafe extern "C" fn volumemonitor_mount_added<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, mount: *mut GMount)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.mount_added()
}

unsafe extern "C" fn volumemonitor_mount_changed<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, mount: *mut GMount)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.mount_changed()
}

unsafe extern "C" fn volumemonitor_mount_pre_unmount<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, mount: *mut GMount)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.mount_pre_unmount()
}

unsafe extern "C" fn volumemonitor_mount_removed<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, mount: *mut GMount)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.mount_removed()
}

unsafe extern "C" fn volumemonitor_volume_added<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, volume: *mut GVolume)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.volume_added()
}

unsafe extern "C" fn volumemonitor_volume_changed<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, volume: *mut GVolume)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.volume_changed()
}

unsafe extern "C" fn volumemonitor_volume_removed<T: VolumeMonitorBase>
(ptr: *mut GVolumeMonitor, volume: *mut GVolume)
where
    T::ImplType: VolumeMonitorImpl<T>
{
    callback_guard!();
    floating_reference_guard!(ptr);
    let volumemonitor = &*(ptr as *mut T::InstanceStructType);
    let wrap: T = from_glib_borrow(ptr as *mut T::InstanceStructType);
    let imp = volumemonitor.get_impl();
    imp.volume_removed()
}