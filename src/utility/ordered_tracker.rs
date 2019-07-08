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

pub struct OrderedTracker<T> {
    slots: Vec<Slot<T>>,
    len: u32,
    free: u32,
}

impl<T> OrderedTracker<T> {
    pub fn new() -> OrderedTracker<T> {
        OrderedTracker {
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

    pub fn insert(&mut self, insert: usize) -> Key<T> {
        let b_index: u32;
        let a_version: u16;
        if self.free > 0 {
            self.free -= 1;
            b_index = self.slots[self.len as usize].b_index;
            a_version = self.slots[b_index as usize].a_version;
            // Free slot, insert at end
            self.slots[b_index as usize].a_index = insert as u32;
        } else {
            a_version = 1;
            b_index = self.len;
            // Full, insert at end
            self.slots.push(Slot {
                a_index: insert as u32,
                a_version: 1,
                b_index: self.len,
                phantom: PhantomData,
            });
        }
        // Free slot/full, insert inside
        let mut cursor = self.len as usize;
        while cursor > insert {
            let next_b_index = self.slots[cursor - 1].b_index;
            self.slots[cursor].b_index = next_b_index;
            self.slots[next_b_index as usize].a_index = cursor as u32;
            cursor -= 1;
        }
        self.slots[insert].b_index = b_index;
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
        let remove = key.index as usize;
        let a_index = self.slots[remove].a_index;
        self.slots[remove].a_version += 1;
        self.len -= 1;
        self.free += 1;

        let mut cursor = remove;
        while cursor < self.len as usize {
            let next_b_index = self.slots[cursor + 1].b_index;
            self.slots[cursor].b_index = next_b_index;
            self.slots[next_b_index as usize].a_index = cursor as u32;
            cursor += 1;
        }
        self.slots[self.len as usize].b_index = remove as u32;
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

    #[test]
    fn insert_continuous_get() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);

        assert_eq!(map.get(first), 0);
        assert_eq!(map.get(second), 1);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn insert_before_get() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(0);
        let third = map.insert(1);

        assert_eq!(map.get(first), 2);
        assert_eq!(map.get(second), 0);
        assert_eq!(map.get(third), 1);
        assert_eq!(map.len, 3);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn insert_clear() {
        let mut map = OrderedTracker::<usize>::new();
        let _first = map.insert(0);
        map.clear();

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
    }

    #[test]
    #[should_panic]
    fn insert_clear_old_key_panics() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        map.clear();

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.get(first); // Panic!
    }

    #[test]
    fn insert_remove_last() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        let index = map.remove(second);

        assert_eq!(map.get(first), 0);
        assert_eq!(index, 1);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    fn insert_remove_first() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        let index = map.remove(first);

        assert_eq!(map.get(second), 0);
        assert_eq!(index, 0);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 1);
    }

    #[test]
    #[should_panic]
    fn insert_remove_get_old_get_panics() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let _index = map.remove(first);

        assert_eq!(map.len, 0);
        assert_eq!(map.free, 1);
        map.get(first); // Panic!
    }

    #[test]
    fn insert_remove_insert() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        let index = map.remove(first);
        let third = map.insert(0);

        assert_eq!(map.get(second), 1);
        assert_eq!(map.get(third), 0);
        assert_eq!(index, 0);
        assert_eq!(map.len, 2);
        assert_eq!(map.free, 0);
    }

    #[test]
    fn insert_remove_all_but_middle() {
        let mut map = OrderedTracker::<usize>::new();
        let first = map.insert(0);
        let second = map.insert(1);
        let third = map.insert(2);
        let first_remove = map.remove(first);
        let third_remove = map.remove(third);

        assert_eq!(map.get(second), 0);
        assert_eq!(first_remove, 0);
        assert_eq!(third_remove, 1);
        assert_eq!(map.len, 1);
        assert_eq!(map.free, 2);
    }
}
