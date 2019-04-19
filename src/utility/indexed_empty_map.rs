#[derive(Copy, Clone, Debug)]
pub struct EmptyKey {
    index: u32,
    version: u32,
}

#[derive(Copy, Clone, Debug)]
struct EmptySlot {
    a_index: u32,
    a_version: u32,
    b_index: u32,
}

pub struct IndexedEmptyMap {
    slots: Vec<EmptySlot>,
    len: u32,
    free: u32,
}

impl IndexedEmptyMap {
    pub fn new() -> IndexedEmptyMap {
        IndexedEmptyMap {
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

    pub fn add(&mut self) -> EmptyKey {
        let index: u32;
        let version: u32;
        if self.free > 0 {
            self.free -= 1;
            {
                let slot = unsafe { self.slots.get_unchecked_mut(self.len as usize) };
                index = slot.b_index;
                slot.b_index = index;
            }
            {
                let slot = unsafe { self.slots.get_unchecked_mut(index as usize) };
                version = slot.a_version;
                slot.a_index = self.len;
            }
        } else {
            self.slots.push(EmptySlot {
                a_index: self.len,
                a_version: 1,
                b_index: self.len,
            });
            index = self.len;
            version = 1;
        };
        self.len += 1;
        EmptyKey {
            index: index,
            version: version,
        }
    }

    pub fn remove(&mut self, key: EmptyKey) -> usize {
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

    pub fn get(&self, key: EmptyKey) -> usize {
        let index = key.index as usize;
        if key.version != self.slots[index].a_version {
            panic!("Unable to get: key version does not match.");
        }
        unsafe { self.slots.get_unchecked(index).a_index as usize }
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
        let mut map = IndexedEmptyMap::new();
        let first = map.add();

        assert_eq!(map.get(first), 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_clear() {
        let mut map = IndexedEmptyMap::new();
        let _first = map.add();
        map.clear();

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_get_second() {
        let mut map = IndexedEmptyMap::new();
        let _first = map.add();
        let second = map.add();

        assert_eq!(map.get(second), 1);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_twice_get_first() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        let _second = map.add();

        assert_eq!(map.get(first), 0);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn add_remove() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    #[should_panic]
    fn add_remove_old_key_panic() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.remove(first); // Panic!
    }

    #[test]
    #[should_panic]
    fn add_get_old_key_panic() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.get(first); // Panic!
    }

    #[test]
    fn add_twice_remove_second() {
        let mut map = IndexedEmptyMap::new();
        let _first = map.add();
        let second = map.add();
        let index = map.remove(second);

        assert_eq!(index, 1);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        let _second = map.add();
        let index = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_twice_remove_first_swaps() {
        let mut map = IndexedEmptyMap::new();
        let first = map.add();
        let second = map.add();
        map.remove(first);

        assert_eq!(map.get(second), 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn add_thrice_remove_first_swaps_ignores_second() {
        let mut map = IndexedEmptyMap::new();
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
        let mut map = IndexedEmptyMap::new();
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

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let mut map = IndexedEmptyMap::new();

        bench.iter(|| {
            for _ in 0..10 {
                let a = map.add();
                black_box(map.get(a));
                black_box(map.remove(a));
            }
        });
    }

    #[bench]
    fn bench_get(bench: &mut Bencher) {
        let mut map = IndexedEmptyMap::new();
        let a = map.add();

        bench.iter(|| {
            for _ in 0..10 {
                black_box(map.get(a));
            }
        });
    }
}
