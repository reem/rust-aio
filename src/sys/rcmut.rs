use std::cell::UnsafeCell;
use std::rc::Rc;
use std::mem;

/// A reference counted smart pointer with unrestricted mutability.
#[derive(Clone)]
pub struct RcMut<T> {
    inner: Rc<UnsafeCell<T>>
}

impl<T> RcMut<T> {
    /// Create a new RcMut for a value.
    pub fn new(val: T) -> RcMut<T> {
        RcMut {
            inner: Rc::new(UnsafeCell::new(val))
        }
    }

    /// Get a reference to the value.
    pub unsafe fn borrow(&self) -> &T {
        mem::transmute(self.inner.get())
    }

    /// Get a mutable reference to the value.
    pub unsafe fn borrow_mut(&mut self) -> &mut T {
        mem::transmute(self.inner.get())
    }
}

