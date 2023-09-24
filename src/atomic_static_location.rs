use std::panic::Location;
use std::sync::atomic::{AtomicPtr, Ordering};

/// An atomic pointer on a static Location.
#[derive(Debug)]
pub struct AtomicStaticLocation {
    inner: AtomicPtr<Location<'static>>,
}

impl AtomicStaticLocation {
    /// Construct an [`AtomicStaticLocation`]
    pub fn new(initial: &'static Location<'static>) -> Self {
        let inner = AtomicPtr::new(initial as *const _ as *mut _);
        AtomicStaticLocation { inner }
    }

    /// Read the Location value atomically.
    pub fn load(&self, ordering: Ordering) -> &'static Location<'static> {
        let ptr = self.inner.load(ordering);
        unsafe { &*ptr }
    }

    /// Store a new Location value atomically.
    pub fn store(&self, value: &'static Location<'static>, ordering: Ordering) {
        self.inner.store(value as *const _ as *mut _, ordering);
    }

    /// Swap the Location value atomically.
    pub fn swap(
        &self,
        value: &'static Location<'static>,
        ordering: Ordering,
    ) -> &'static Location<'static> {
        let new_ptr = value as *const _ as *mut _;
        let old_ptr = self.inner.swap(new_ptr, ordering);
        unsafe { &*old_ptr }
    }
}

#[test]
fn test_atomic_location() {
    #[track_caller]
    fn get_caller_location() -> &'static Location<'static> {
        Location::caller()
    }

    let location1 = get_caller_location();
    let location2 = get_caller_location();
    assert_eq!(location1.file(), file!());
    assert_eq!(location1.line(), 43);
    assert_eq!(location1.column(), 21);

    let atomic_static_location = AtomicStaticLocation::new(location1);

    let old_value = atomic_static_location.swap(location2, Ordering::Relaxed);
    assert_eq!(old_value, location1);

    let current_value = atomic_static_location.load(Ordering::Relaxed);
    assert_eq!(current_value, location2);
}
