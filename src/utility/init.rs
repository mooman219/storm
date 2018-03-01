use std::cell::UnsafeCell;
use std::ops::Deref;

//
// StaticStack
//

/// Helper union to reserve space for T without initializing it.
union StackStore<T: Sync> {
    value: T,
    dummy: bool,
}

/// Stores T on the stack.
pub struct StaticStack<T: Sync> {
    store: UnsafeCell<StackStore<T>>,
    initializer: fn() -> T,
}

unsafe impl<T: Sync> Sync for StaticStack<T> {}

impl<T: Sync> StaticStack<T> {
    pub const fn new(initializer: fn() -> T) -> StaticStack<T> {
        StaticStack {
            store: UnsafeCell::new(StackStore { dummy: false }),
            initializer: initializer,
        }
    }

    pub fn init(&self) {
        unsafe {
            let value = (self.initializer)();
            let pointer = self.store.get();
            *pointer = StackStore { value: value };
        }
    }
}

impl<T: Sync> Deref for StaticStack<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.store.get()).value }
    }
}

//
// StaticHeap
//

/// Stores T on the heap.
pub struct StaticHeap<T: Sync> {
    store: UnsafeCell<*const T>,
    initializer: fn() -> T,
}

unsafe impl<T: Sync> Sync for StaticHeap<T> {}

impl<T: Sync> StaticHeap<T> {
    pub const fn new(initializer: fn() -> T) -> StaticHeap<T> {
        StaticHeap {
            store: UnsafeCell::new(0 as *const _),
            initializer: initializer,
        }
    }

    pub fn init(&self) {
        unsafe {
            let value = (self.initializer)();
            let pointer = self.store.get();
            *pointer = Box::into_raw(Box::new(value));
        }
    }
}

impl<T: Sync> Deref for StaticHeap<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &**self.store.get() }
    }
}
