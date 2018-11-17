// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Module that contains the basic infrastructure for subclassing `GObject`
use glib_ffi;
use gobject_ffi;

use std::mem;
use std::ops;
use std::ptr;

use glib;
use glib::translate::*;

use object::ObjectImpl;

/// A newly registered `glib::Type` that is currently still being initialized
///
/// This allows running additional type-setup functions, e.g. for implementing
/// interfaces on the type
#[derive(Debug, PartialEq, Eq)]
pub struct InitializingType(glib::Type);

impl ops::Deref for InitializingType {
    type Target = glib::Type;

    fn deref(&self) -> &glib::Type {
        &self.0
    }
}

/// Trait implemented by structs that implement a `GObject` C instance struct
///
/// The struct must be `#[repr(C)]` and have the parent type's instance struct
/// as the first field.
///
/// See [`simple::InstanceStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the instance struct.
///
/// [`simple::InstanceStruct`]: ../simple/struct.InstanceStruct.html
pub unsafe trait InstanceStruct: Sized + 'static {
    /// Corresponding object subclass type for this instance struct
    type Type: ObjectSubclass;

    /// Returns the implementation for from this instance struct, that
    /// is the implementor of [`ObjectImpl`] or subtraits.
    ///
    /// [`ObjectImpl`]: ../object/trait.ObjectImpl.html
    fn get_impl(&self) -> &Self::Type {
        unsafe {
            let data = Self::Type::type_data();
            let private_offset = data.as_ref().private_offset;
            let ptr: *const u8 = self as *const _ as *const u8;
            let priv_ptr = ptr.offset(private_offset);
            let imp = priv_ptr as *const Option<Self::Type>;

            (*imp).as_ref().expect("No private struct")
        }
    }

    /// Returns the class struct for this specific instance
    fn get_class(&self) -> &<Self::Type as ObjectSubclass>::Class {
        unsafe { &**(self as *const _ as *const *const <Self::Type as ObjectSubclass>::Class) }
    }
}

/// Trait implemented by structs that implement a `GObject` C class struct
///
/// The struct must be `#[repr(C)]` and have the parent type's class struct
/// as the first field.
///
/// See [`simple::ClassStruct`] for a basic implementation of this that can
/// be used most of the time and should only not be used if additional fields are
/// required in the class struct, e.g. for declaring new virtual methods.
///
/// [`simple::ClassStruct`]: ../simple/struct.ClassStruct.html
pub unsafe trait ClassStruct: Sized + 'static {
    /// Corresponding object subclass type for this class struct
    type Type: ObjectSubclass;
}

/// Trait for declaring the subclass relationship between classes
///
/// This is the class version of `glib::IsA`.
// TODO: This should be in glib-rs
pub unsafe trait IsAClass<T> {}

/// Trait for mapping a class struct type to its corresponding instance type
// TODO: This should be in glib-rs
pub unsafe trait IsClassFor<T> {}

/// Type-specific data that is filled in during type creation
pub struct TypeData {
    #[doc(hidden)]
    pub type_: glib::Type,
    #[doc(hidden)]
    pub parent_class: glib_ffi::gpointer,
    #[doc(hidden)]
    pub interfaces: *const Vec<(glib_ffi::GType, glib_ffi::gpointer)>,
    #[doc(hidden)]
    pub private_offset: isize,
}

unsafe impl Send for TypeData {}
unsafe impl Sync for TypeData {}

impl TypeData {
    /// Returns the type ID
    pub fn get_type(&self) -> glib::Type {
        self.type_
    }

    /// Returns a pointer to the native parent class
    ///
    /// This is used for chaining up to the parent class' implementation
    /// of virtual methods
    pub fn get_parent_class(&self) -> glib_ffi::gpointer {
        self.parent_class
    }

    /// Returns a pointer to the interface implementation
    ///
    /// This is used for calling the interface method implementations
    pub fn get_interface(&self, type_: glib_ffi::GType) -> glib_ffi::gpointer {
        unsafe {
            if self.interfaces.is_null() {
                return ptr::null_mut();
            }

            for &(t, p) in &(*self.interfaces) {
                if t == type_ {
                    return p;
                }
            }

            ptr::null_mut()
        }
    }

    /// Returns the offset of the private struct in bytes relative to the
    /// beginning of the instance struct
    pub fn get_private_offset(&self) -> isize {
        self.private_offset
    }
}

#[macro_export]
/// Macro for boilerplate of [`ObjectSubclass`] implementations
///
/// [`ObjectSubclass`]: types/trait.ObjectSubclass.html
macro_rules! object_subclass {
    () => {
        fn type_data() -> ::std::ptr::NonNull<$crate::TypeData> {
            static mut DATA: $crate::TypeData = $crate::TypeData {
                type_: $crate::glib::Type::Invalid,
                parent_class: ::std::ptr::null_mut(),
                interfaces: ::std::ptr::null_mut(),
                private_offset: 0,
            };

            unsafe { ::std::ptr::NonNull::new_unchecked(&mut DATA) }
        }
    };
}

/// The central trait for subclassing a `GObject` type
///
/// Links together the type name, parent type and the instance and
/// class structs for type registration and allows subclasses to
/// hook into various steps of the type registration and initialization.
///
/// See [`register_type`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_type`]: fn.register_type.html
pub trait ObjectSubclass: ObjectImpl + Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Parent Rust type to inherit from
    type ParentType: glib::IsA<glib::Object>
        + glib::translate::FromGlibPtrBorrow<
            *mut <Self::ParentType as glib::wrapper::Wrapper>::GlibType,
        >;

    /// The C instance struct
    ///
    /// See [`simple::InstanceStruct`] for an basic instance struct that should be
    /// used in most cases
    ///
    /// [`simple::InstanceStruct`]: ../simple/struct.InstanceStruct.html
    // TODO: Should default to simple::InstanceStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Instance: InstanceStruct<Type = Self>;

    /// The C class struct
    ///
    /// See [`simple::ClassStruct`] for an basic instance struct that should be
    /// used in most cases
    ///
    /// [`simple::ClassStruct`]: ../simple/struct.ClassStruct.html
    // TODO: Should default to simple::ClassStruct<Self> once associated
    // type defaults are stabilized https://github.com/rust-lang/rust/issues/29661
    type Class: ClassStruct<Type = Self>;

    /// Storage for the type-specific data used during registration
    ///
    /// This is usually generated by the [`object_subclass!`] macro.
    ///
    /// [`object_subclass!`]: ../macro.object_subclass.html
    fn type_data() -> ptr::NonNull<TypeData>;

    /// Returns the `glib::Type` ID of the subclass
    ///
    /// This will panic if called before the type was registered at
    /// runtime with the `GObject` type system.
    fn static_type() -> glib::Type {
        unsafe {
            let data = Self::type_data();
            let type_ = data.as_ref().get_type();
            assert_ne!(type_, glib::Type::Invalid);

            type_
        }
    }

    /// Additional type initialization
    ///
    /// This is called right after the type was registered and allows
    /// subclasses to do additional type-specific initialization, e.g.
    /// for implementing `GObject` interfaces.
    ///
    /// Optional
    fn type_init(_type_: &InitializingType) {}

    /// Class initialization
    ///
    /// This is called after `type_init` and before the first instance
    /// of the subclass is created. Subclasses can use this to do class-
    /// specific initialization, e.g. for installing properties or signals
    /// on the class or calling class methods.
    ///
    /// Optional
    fn class_init(_klass: &mut Self::Class) {}

    /// Constructor
    ///
    /// This is called during object instantiation before further subclasses
    /// are initialized, and should return a new instance of the subclass
    /// private struct.
    fn new(obj: &Self::ParentType) -> Self;
}

unsafe extern "C" fn class_init<T: ObjectSubclass>(
    klass: glib_ffi::gpointer,
    _klass_data: glib_ffi::gpointer,
) {
    let mut data = T::type_data();

    // We have to update the private struct offset once the class is actually
    // being initialized
    {
        let mut private_offset = data.as_ref().private_offset as i32;
        gobject_ffi::g_type_class_adjust_private_offset(klass, &mut private_offset);
        (*data.as_mut()).private_offset = private_offset as isize;
    }

    // Set trampolines for the basic GObject virtual methods
    {
        let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);

        gobject_klass.finalize = Some(finalize::<T>);
    }

    // And finally peek the parent class struct (containing the parent class'
    // implementations of virtual methods for chaining up), and call the subclass'
    // class initialization function
    {
        let klass = &mut *(klass as *mut T::Class);
        let parent_class =
            gobject_ffi::g_type_class_peek_parent(klass as *mut _ as glib_ffi::gpointer)
                as *mut <T::ParentType as glib::wrapper::Wrapper>::GlibClassType;
        assert!(!parent_class.is_null());

        (*data.as_mut()).parent_class = parent_class as glib_ffi::gpointer;

        T::class_init(klass);
    }
}

unsafe extern "C" fn instance_init<T: ObjectSubclass>(
    obj: *mut gobject_ffi::GTypeInstance,
    _klass: glib_ffi::gpointer,
) {
    floating_reference_guard!(obj);
    let rs_instance: T::ParentType =
        from_glib_borrow(obj as *mut <T::ParentType as glib::wrapper::Wrapper>::GlibType);

    // Get offset to the storage of our private struct, create it
    // and actually store it in that place
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let imp = T::new(&rs_instance);

    ptr::write(imp_storage, Some(imp));
}

unsafe extern "C" fn finalize<T: ObjectSubclass>(obj: *mut gobject_ffi::GObject) {
    floating_reference_guard!(obj);

    // Retrieve the private struct, take it out of its storage and
    // drop it for freeing all associated memory
    let mut data = T::type_data();
    let private_offset = (*data.as_mut()).private_offset;
    let ptr: *mut u8 = obj as *mut _ as *mut u8;
    let priv_ptr = ptr.offset(private_offset);
    let imp_storage = priv_ptr as *mut Option<T>;

    let imp = (*imp_storage).take().expect("No private struct");
    drop(imp);

    // Chain up to the parent class' finalize implementation, if any
    let parent_class = &*(data.as_ref().get_parent_class() as *const gobject_ffi::GObjectClass);
    if let Some(ref func) = parent_class.finalize {
        func(obj);
    }
}

#[macro_export]
/// Macro for defining a `get_type` function
///
/// This returns a `glib::Type` and registers `Self` via [`register_type`]
/// the first time it is called.
///
/// [`register_type`]: types/fn.register_type.html
macro_rules! object_get_type {
    () => {
        pub fn get_type() -> $crate::glib::Type {
            use std::sync::Once;
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                $crate::register_type::<Self>();
            });

            Self::static_type()
        }
    };
}

/// Register a `glib::Type` ID for `T`
///
/// This must be called only once and will panic on a second call.
///
/// See [`object_get_type!`] for defining a function that ensures that
/// this is only called once and returns the type id.
///
/// [`object_get_type!`]: ../macro.object_get_type.html
pub fn register_type<T: ObjectSubclass>() -> glib::Type {
    unsafe {
        use std::ffi::CString;

        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<T::Class>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(class_init::<T>),
            class_finalize: None,
            class_data: ptr::null_mut(),
            instance_size: mem::size_of::<T::Instance>() as u16,
            n_preallocs: 0,
            instance_init: Some(instance_init::<T>),
            value_table: ptr::null(),
        };

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = from_glib(gobject_ffi::g_type_register_static(
            <T::ParentType as glib::StaticType>::static_type().to_glib(),
            type_name.as_ptr(),
            &type_info,
            0,
        ));

        let mut data = T::type_data();
        (*data.as_mut()).type_ = type_;
        let private_offset =
            gobject_ffi::g_type_add_instance_private(type_.to_glib(), mem::size_of::<Option<T>>());
        (*data.as_mut()).private_offset = private_offset as isize;

        T::type_init(&InitializingType(type_));

        type_
    }
}
