use std::alloc;
use std::alloc::Layout;
use std::mem;
use std::ptr;

struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        // !0 is usize::MAX. This branch should be stripped at compile time.
        let cap = if mem::size_of::<T>() == 0 {
            !0
        } else {
            0
        };

        // Unique::empty() doubles as "unallocated" and "zero-sized allocation"
        RawVec {
            ptr: ptr::null_mut(),
            cap: cap,
        }
    }

    fn grow(&mut self, min_capacity: usize) {
        if min_capacity > self.cap {
            unsafe {
                let elem_size = mem::size_of::<T>();

                // since we set the capacity to usize::MAX when elem_size is
                // 0, getting to here necessarily means the Vec is overfull.
                assert!(elem_size != 0, "capacity overflow");

                let new_cap = 2 * min_capacity;
                let new_size = min_capacity.checked_mul(mem::size_of::<T>()).expect("capacity overflow");
                let new_layout = Layout::array::<T>(new_size).unwrap();

                let ptr = if self.cap == 0 {
                    alloc::alloc(new_layout)
                } else {
                    let old_size = self.cap.checked_mul(mem::size_of::<T>()).expect("capacity overflow");
                    let old_layout = Layout::array::<T>(old_size).unwrap();
                    alloc::realloc(self.ptr as *mut _, old_layout, new_layout.size())
                };

                // If allocate or reallocate fail, oom
                if ptr.is_null() {
                    alloc::handle_alloc_error(new_layout)
                }

                self.ptr = ptr as *mut _;
                self.cap = new_cap;
            }
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            unsafe {
                let size = self.cap.checked_mul(mem::size_of::<T>()).expect("capacity overflow");
                let layout = Layout::array::<T>(size).unwrap();
                alloc::dealloc(self.ptr as *mut _, layout);
            }
        }
    }
}

pub struct Storage<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage {
            buf: RawVec::new(),
            len: 0,
        }
    }

    #[inline]
    pub fn ptr(&self) -> *mut T {
        self.buf.ptr
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn push(&mut self, elem: T) {
        self.buf.grow(self.len + 1);
        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    #[inline]
    pub fn push_range(&mut self, elems: Vec<T>) {
        self.buf.grow(self.len + elems.len());
        unsafe {
            ptr::copy_nonoverlapping(elems.as_ptr(), self.ptr().add(self.len), elems.len());
        }
        self.len += elems.len();
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    #[inline]
    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        self.buf.grow(self.len + 1);
        unsafe {
            if index < self.len {
                ptr::copy(self.ptr().add(index), self.ptr().add(index + 1), self.len - index);
            }
            ptr::write(self.ptr().add(index), elem);
        }
        self.len += 1;
    }

    #[inline]
    pub fn insert_range(&mut self, index: usize, elems: Vec<T>) {
        assert!(index <= self.len, "index out of bounds");
        self.buf.grow(self.len + elems.len());
        unsafe {
            if index < self.len {
                ptr::copy(self.ptr().add(index), self.ptr().add(index + elems.len()), self.len - index);
            }
            ptr::copy_nonoverlapping(elems.as_ptr(), self.ptr().add(index), elems.len());
        }
        self.len += elems.len();
    }

    #[inline]
    pub fn replace(&mut self, index: usize, elem: T) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe { ptr::replace(self.ptr().add(index), elem) }
    }

    #[inline]
    pub fn update(&mut self, index: usize, elem: T) {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.ptr().add(index).drop_in_place();
            ptr::write(self.ptr().add(index), elem);
        }
    }

    #[inline]
    pub unsafe fn update_without_drop(&mut self, index: usize, elem: T) {
        assert!(index < self.len, "index out of bounds");
        ptr::write(self.ptr().add(index), elem);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        unsafe {
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(self.ptr().add(index + 1), self.ptr().add(index), self.len - index);
            result
        }
    }

    #[inline]
    pub fn remove_without_drop(&mut self, index: usize) {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        unsafe {
            ptr::copy(self.ptr().add(index + 1), self.ptr().add(index), self.len - index);
        }
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        unsafe {
            let result = ptr::read(self.ptr().add(index));
            ptr::copy_nonoverlapping(self.ptr().add(index), self.ptr().add(self.len), 1);
            result
        }
    }

    #[inline]
    pub unsafe fn swap_remove_without_drop(&mut self, index: usize) {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        ptr::copy_nonoverlapping(self.ptr().add(index), self.ptr().add(self.len), 1);
    }

    #[inline]
    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {}
    }

    #[inline]
    pub unsafe fn clear_without_drop(&mut self) {
        self.len = 0;
    }
}

impl<T> Drop for Storage<T> {
    fn drop(&mut self) {
        self.clear();
        // allocation is handled by RawVec
    }
}
