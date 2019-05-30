use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Key<T> {
    index: u32,
    version: u16,
    phantom: PhantomData<T>,
}

#[derive(Copy, Clone, Debug)]
struct Slot<T> {
    a_index: u32,
    a_version: u16,
    b_index: u32,
    phantom: PhantomData<T>,
}

pub struct UnorderedTracker<T> {
    slots: Vec<Slot<T>>,
    len: u32,
    free: u32,
}

impl<T> UnorderedTracker<T> {
    pub fn new() -> UnorderedTracker<T> {
        UnorderedTracker {
            slots: Vec::with_capacity(64),
            len: 0,
            free: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn clear(&mut self) {
        let mut counter = 0;
        for slot in self.slots.iter_mut() {
            slot.a_version += 1;
            slot.a_index = counter;
            slot.b_index = counter;
            counter += 1;
        }
        self.len = 0;
        self.free = self.slots.len() as u32;
    }

    pub fn add(&mut self) -> Key<T> {
        let b_index: u32;
        let a_version: u16;
        if self.free > 0 {
            self.free -= 1;
            b_index = self.slots[self.len as usize].b_index;
            let slot = unsafe { self.slots.get_unchecked_mut(b_index as usize) };
            a_version = slot.a_version;
            slot.a_index = self.len;
        } else {
            self.slots.push(Slot {
                a_index: self.len,
                a_version: 1,
                b_index: self.len,
                phantom: PhantomData,
            });
            b_index = self.len;
            a_version = 1;
        };
        self.len += 1;
        Key {
            index: b_index,
            version: a_version,
            phantom: PhantomData,
        }
    }

    pub fn remove(&mut self, key: Key<T>) -> usize {
        if key.version != self.slots[key.index as usize].a_version {
            panic!("Unable to remove: token version does not match.");
        }
        self.slots[key.index as usize].a_version += 1;
        self.len -= 1;
        let a_index = self.slots[key.index as usize].a_index;
        if a_index < self.len {
            let b_index = self.slots[self.len as usize].b_index;
            self.slots[a_index as usize].b_index = b_index;
            self.slots[b_index as usize].a_index = a_index;
        }
        // Free slot book-keeping
        self.slots[self.len as usize].b_index = key.index;
        self.free += 1;

        a_index as usize
    }

    pub fn get(&self, key: Key<T>) -> usize {
        let index = key.index as usize;
        if key.version != self.slots[index].a_version {
            panic!("Unable to get: key version does not match.");
        }
        self.slots[index].a_index as usize
    }
}

// ////////////////////////////////////////////////////////////////////////////
// Tests
// ////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use test::black_box;
    use test::Bencher;

    #[test]
    fn add_get() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();

        assert_eq!(map.get(first), 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_clear() {
        let mut map = UnorderedTracker::<usize>::new();
        let _first = map.add();
        map.clear();

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_get_second() {
        let mut map = UnorderedTracker::<usize>::new();
        let _first = map.add();
        let second = map.add();

        assert_eq!(map.get(second), 1);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_twice_get_first() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let _second = map.add();

        assert_eq!(map.get(first), 0);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_remove() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    #[should_panic]
    fn add_remove_old_key_panic() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.remove(first); // Panic!
    }

    #[test]
    #[should_panic]
    fn add_get_old_key_panic() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.get(first); // Panic!
    }

    #[test]
    fn add_twice_remove_second() {
        let mut map = UnorderedTracker::<usize>::new();
        let _first = map.add();
        let second = map.add();
        let index = map.remove(second);

        assert_eq!(index, 1);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let _second = map.add();
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first_swaps() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let second = map.add();
        map.remove(first);

        assert_eq!(map.get(second), 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_thrice_remove_first_swaps_ignores_second() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let second = map.add();
        let _third = map.add();
        map.remove(first);

        assert_eq!(map.get(second), 1);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first_add() {
        let mut map = UnorderedTracker::<usize>::new();
        let first = map.add();
        let _second = map.add();
        map.remove(first);
        let third = map.add();

        assert_eq!(map.get(third), 1);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    // ////////////////////////////////////////////////////////////////////////////
    // Benches
    // ////////////////////////////////////////////////////////////////////////////

    const ITERATIONS: usize = 1000;

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let mut map = UnorderedTracker::<usize>::new();

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                let a = map.add();
                black_box(map.get(a));
                black_box(map.remove(a));
            }
        });
    }

    #[bench]
    fn bench_get(bench: &mut Bencher) {
        let mut map = UnorderedTracker::<usize>::new();
        let a = map.add();

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                black_box(map.get(a));
            }
        });
    }

    #[bench]
    fn bench_add(bench: &mut Bencher) {
        let mut map = UnorderedTracker::<usize>::new();

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                black_box(map.add());
            }
        });
    }
}
