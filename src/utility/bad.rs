use core::ops::{Deref, DerefMut};

struct Inner<T> {
    value: T,
    count: usize,
}

pub struct UnsafeShared<T> {
    inner: *mut Inner<T>,
}

impl<T> UnsafeShared<T> {
    pub fn new(value: T) -> UnsafeShared<T> {
        let inner = Inner {
            value: value,
            count: 1,
        };
        UnsafeShared {
            inner: Box::into_raw(Box::new(inner)),
        }
    }

    pub fn count(&self) -> usize {
        let inner = unsafe { &mut (*self.inner) };
        inner.count
    }
}

impl<T> Clone for UnsafeShared<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &mut (*self.inner) };
        inner.count += 1;
        UnsafeShared {
            inner: self.inner,
        }
    }
}

impl<T> Drop for UnsafeShared<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut (*self.inner) };
        inner.count -= 1;
        if inner.count == 0 {
            unsafe { Box::from_raw(self.inner) };
        }
    }
}

impl<T> Deref for UnsafeShared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner).value }
    }
}

impl<T> DerefMut for UnsafeShared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.inner).value }
    }
}
