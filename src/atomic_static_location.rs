use std::sync::atomic::{AtomicPtr, Ordering};

/// Alias for `std::panic::Location<'static>`.
pub type Location = std::panic::Location<'static>;

/// An atomic pointer on an optional static Location.
#[derive(Debug, Default)]
pub struct AtomicStaticLocation {
    inner: AtomicPtr<Location>,
}

impl AtomicStaticLocation {
    /// Construct an [`AtomicStaticLocation`]
    pub const fn new(initial: Option<&'static Location>) -> Self {
        let initial = match initial {
            Some(r) => r as *const _ as *mut _,
            None => std::ptr::null_mut(),
        };
        let inner = AtomicPtr::new(initial);
        AtomicStaticLocation { inner }
    }

    /// Read the Location value atomically.
    pub fn load(&self, ordering: Ordering) -> Option<&'static Location> {
        let ptr = self.inner.load(ordering);
        unsafe { ptr.as_ref() }
    }

    /// Store a new Location value atomically.
    pub fn store(&self, value: Option<&'static Location>, ordering: Ordering) {
        let value = match value {
            Some(r) => r as *const _ as *mut _,
            None => std::ptr::null_mut(),
        };
        self.inner.store(value, ordering);
    }

    /// Swap the Location value atomically.
    pub fn swap(
        &self,
        value: Option<&'static Location>,
        ordering: Ordering,
    ) -> Option<&'static Location> {
        let new_ptr = match value {
            Some(r) => r as *const _ as *mut _,
            None => std::ptr::null_mut(),
        };
        let old_ptr = self.inner.swap(new_ptr, ordering);
        unsafe { old_ptr.as_ref() }
    }
}

#[test]
fn test_atomic_location() {
    #[track_caller]
    fn get_caller_location() -> &'static Location {
        Location::caller()
    }

    let location1 = get_caller_location();
    let location2 = get_caller_location();
    assert_eq!(location1.file(), file!());
    assert_eq!(location1.line(), 60);
    assert_eq!(location1.column(), 21);

    let atomic_static_location = AtomicStaticLocation::new(Some(location1));

    let old_value = atomic_static_location.swap(Some(location2), Ordering::Relaxed);
    assert_eq!(old_value, Some(location1));

    let current_value = atomic_static_location.load(Ordering::Relaxed);
    assert_eq!(current_value, Some(location2));

    let current_value = atomic_static_location.swap(None, Ordering::Relaxed);
    assert_eq!(current_value, Some(location2));

    let current_value = atomic_static_location.load(Ordering::Relaxed);
    assert_eq!(current_value, None);
}
