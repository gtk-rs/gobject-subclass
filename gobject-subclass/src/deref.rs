#[macro_export]
macro_rules! gobject_subclass_deref(

    ($name:ident, $base:ident) => {
        gobject_subclass_deref!($name, imp::$name, $base);
    };

    ($name:ident, $target:ty, $base:ident) => {
        use std::ops::Deref;

        impl Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    let base: $base = from_glib_borrow(self.to_glib_none().0);
                    let imp = base.get_impl();
                    let imp = imp.downcast_ref::<$target>().unwrap();
                    // Cast to a raw pointer to get us an appropriate lifetime: the compiler
                    // can't know that the lifetime of base is the same as the one of self
                    &*(imp as *const $target)
                }
            }
        }
    }
);
