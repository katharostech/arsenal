// Necessary for some macros because they won't generate docs
#![allow(missing_docs)]

use safer_ffi::prelude::*;

use ::std::{
    any::{Any, TypeId},
    sync::Arc,
};

mod ty {
    use safer_ffi::derive_ReprC;

    #[derive_ReprC]
    #[ReprC::opaque]
    pub struct Erased {
        _private: (),
    }
}

/// VTable used to keep track of script type bindings
#[derive_ReprC]
#[ReprC::opaque]
pub struct VTable {
    type_id: fn() -> TypeId,
    clone_arc: unsafe fn(*const ty::Erased) -> FFIObj,
    drop_arc: unsafe fn(*const ty::Erased),
}

/// A ref-counted pointer that can be sent over FFI
#[derive_ReprC]
#[repr(C)]
pub struct FFIObj {
    ptr: *const ty::Erased,
    vtable: &'static VTable,
}

trait HasVTable
where
    Self: Sized + Any + Send + Sync + 'static,
{
    const VTABLE: VTable = VTable {
        type_id: || TypeId::of::<Self>(),

        drop_arc: {
            unsafe fn drop_arc<T: Any + Send + Sync + 'static>(ptr: *const ty::Erased) {
                let ptr: *const T = ptr.cast();
                drop::<Arc<T>>(Arc::from_raw(ptr))
            }
            drop_arc::<Self>
        },

        clone_arc: {
            unsafe fn clone_arc<T: Any + Send + Sync + 'static>(ptr: *const ty::Erased) -> FFIObj {
                let ptr: *const T = ptr.cast();
                let arc_ref: &Arc<T> = &*::core::mem::ManuallyDrop::new(Arc::<T>::from_raw(ptr));
                let owned_clone: Arc<T> = Arc::clone(arc_ref);
                FFIObj {
                    ptr: Arc::into_raw(owned_clone).cast(),
                    vtable: &T::VTABLE
                }
            }
            clone_arc::<Self>
        },
    };
}

impl<T: ?Sized> HasVTable for T where Self: Sized + Any + Send + Sync + 'static {}

impl FFIObj {
    /// Create a new FFI compatible pointer to the given type
    pub fn new<T: Any + Send + Sync + 'static>(value: T) -> Self {
        Self {
            ptr: Arc::into_raw(Arc::new(value)).cast(),
            vtable: &T::VTABLE,
        }
    }

    /// Downcast the FFI object to a specific type
    pub fn downcast_ref<T: Any>(self: &'_ FFIObj) -> Option<&'_ T> {
        if self.is::<T>() {
            unsafe { Some(&*self.ptr.cast()) }
        } else {
            None
        }
    }

    fn is<T: Any>(self: &'_ FFIObj) -> bool {
        (self.vtable.type_id)() == ::core::any::TypeId::of::<T>()
    }
}

impl Clone for FFIObj {
    fn clone(self: &'_ FFIObj) -> FFIObj {
        unsafe { (self.vtable.clone_arc)(self.ptr) }
    }
}

impl Drop for FFIObj {
    fn drop(self: &'_ mut FFIObj) {
        unsafe { (self.vtable.drop_arc)(self.ptr) }
    }
}

unsafe impl Send for FFIObj where Arc<dyn Any + Send + Sync + 'static>: Send {}
unsafe impl Sync for FFIObj where Arc<dyn Any + Send + Sync + 'static>: Sync {}
