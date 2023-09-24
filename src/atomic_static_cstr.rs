use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicPtr, Ordering};

/// An atomic pointer on a 'static C-string
#[derive(Debug)]
pub struct AtomicStaticCStr {
    inner: AtomicPtr<c_char>,
}

impl AtomicStaticCStr {
    /// Construct an [`AtomicStaticCStr`]
    pub fn new(initial: &'static CStr) -> Self {
        let inner = AtomicPtr::new(initial.as_ptr() as *mut _);
        AtomicStaticCStr { inner }
    }

    /// Read the CStr value atomically.
    pub fn load(&self, ordering: Ordering) -> &'static CStr {
        let ptr = self.inner.load(ordering);
        unsafe { CStr::from_ptr(ptr) }
    }

    /// Store a new CStr value atomically.
    pub fn store(&self, value: &'static CStr, ordering: Ordering) {
        self.inner.store(value.as_ptr() as *mut _, ordering);
    }

    /// Swap the CStr value atomically.
    pub fn swap(&self, value: &'static CStr, ordering: Ordering) -> &'static CStr {
        let new_ptr = value.as_ptr() as *mut _;
        let old_ptr = self.inner.swap(new_ptr, ordering);
        unsafe { CStr::from_ptr(old_ptr) }
    }
}

#[test]
fn test_atomic_cstr() {
    const HELLO: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"Hello, world!\0") };
    const NEW: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"New string!\0") };

    let atomic_static_cstr = AtomicStaticCStr::new(HELLO);

    let old_value = atomic_static_cstr.swap(NEW, Ordering::Relaxed);
    assert_eq!(old_value, HELLO);

    let current_value = atomic_static_cstr.load(Ordering::Relaxed);
    assert_eq!(current_value, NEW);
}
