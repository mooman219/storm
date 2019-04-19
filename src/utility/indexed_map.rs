use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Key<T: Copy> {
    index: u32,
    version: u32,
    phantom: PhantomData<T>,
}

#[derive(Copy, Clone)]
struct Slot<T: Copy> {
    a_index: u32,
    a_version: u32,
    b_index: u32,
    b_value: T,
}

pub struct IndexedMap<T: Copy> {
    slots: Vec<Slot<T>>,
    len: u32,
    free: u32,
}

impl<T: Copy> IndexedMap<T> {
    pub fn new() -> IndexedMap<T> {
        IndexedMap {
            slots: Vec::with_capacity(64),
            len: 0,
            free: 0,
        }
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

    pub fn add(&mut self, value: &T) -> Key<T> {
        let index: u32;
        let version: u32;
        if self.free > 0 {
            self.free -= 1;
            {
                let slot = unsafe { self.slots.get_unchecked_mut(self.len as usize) };
                index = slot.b_index;
                slot.b_index = index;
                slot.b_value = *value;
            }
            {
                let slot = unsafe { self.slots.get_unchecked_mut(index as usize) };
                version = slot.a_version;
                slot.a_index = self.len;
            }
        } else {
            self.slots.push(Slot {
                a_index: self.len,
                a_version: 1,
                b_index: self.len,
                b_value: *value,
            });
            index = self.len;
            version = 1;
        };
        self.len += 1;
        Key {
            index: index,
            version: version,
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
            let b_value = self.slots[self.len as usize].b_value;
            self.slots[a_index as usize].b_index = b_index;
            self.slots[a_index as usize].b_value = b_value;
            self.slots[b_index as usize].a_index = a_index;
        }
        // Free slot book-keeping
        self.slots[self.len as usize].b_index = key.index;
        self.free += 1;

        a_index as usize
    }

    pub fn get(&self, key: Key<T>) -> (usize, T) {
        let index = key.index as usize;
        if key.version != self.slots[index].a_version {
            panic!("Unable to get: key version does not match.");
        }
        let slot = unsafe { self.slots.get_unchecked(index).a_index as usize };
        let value = unsafe { self.slots.get_unchecked(slot).b_value };
        (slot, value)
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
        let mut map = IndexedMap::new();
        let first = map.add(&'a');

        assert_eq!(map.get(first), (0, 'a'));
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_clear() {
        let mut map = IndexedMap::new();
        let _first = map.add(&'a');
        map.clear();

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_get_second() {
        let mut map = IndexedMap::new();
        let _first = map.add(&'a');
        let second = map.add(&'b');

        assert_eq!(map.get(second), (1, 'b'));
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_twice_get_first() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let _second = map.add(&'b');

        assert_eq!(map.get(first), (0, 'a'));
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_remove() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    #[should_panic]
    fn add_remove_old_key_panic() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.remove(first); // Panic!
    }

    #[test]
    #[should_panic]
    fn add_get_old_key_panic() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.get(first); // Panic!
    }

    #[test]
    fn add_twice_remove_second() {
        let mut map = IndexedMap::new();
        let _first = map.add(&'a');
        let second = map.add(&'b');
        let index = map.remove(second);

        assert_eq!(index, 1);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let _second = map.add(&'b');
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first_swaps() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let second = map.add(&'b');
        map.remove(first);

        assert_eq!(map.get(second), (0, 'b'));
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_thrice_remove_first_swaps_ignores_second() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let second = map.add(&'b');
        let _third = map.add(&'c');
        map.remove(first);

        assert_eq!(map.get(second), (1, 'b'));
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first_add() {
        let mut map = IndexedMap::new();
        let first = map.add(&'a');
        let _second = map.add(&'b');
        map.remove(first);
        let third = map.add(&'c');

        assert_eq!(map.get(third), (1, 'c'));
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    // ////////////////////////////////////////////////////////////////////////////
    // Benches
    // ////////////////////////////////////////////////////////////////////////////

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let mut map = IndexedMap::new();

        bench.iter(|| {
            for _ in 0..10 {
                let a = map.add(&'a');
                black_box(map.get(a));
                black_box(map.remove(a));
            }
        });
    }

    #[bench]
    fn bench_get(bench: &mut Bencher) {
        let mut map = IndexedMap::new();
        let a = map.add(&'a');

        bench.iter(|| {
            for _ in 0..10 {
                black_box(map.get(a));
            }
        });
    }
}
