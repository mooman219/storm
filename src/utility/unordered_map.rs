use utility::unordered_tracker::*;

pub struct UnorderedMap<T> {
    map: UnorderedTracker<T>,
    values: Vec<T>,
}

impl<T> UnorderedMap<T> {
    pub fn new() -> UnorderedMap<T> {
        UnorderedMap {
            map: UnorderedTracker::new(),
            values: Vec::with_capacity(64),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.values.clear();
    }

    pub fn add(&mut self, value: T) -> Key<T> {
        self.values.push(value);
        self.map.add()
    }

    pub fn remove(&mut self, key: Key<T>) -> (usize, T) {
        let index = self.map.remove(key);
        let value = self.values.swap_remove(index);
        (index, value)
    }

    pub fn get(&self, key: Key<T>) -> (usize, &T) {
        let index = self.map.get(key);
        let value = &self.values[index];
        (index, value)
    }

    pub fn get_mut(&mut self, key: Key<T>) -> (usize, &T) {
        let index = self.map.get(key);
        let value = &mut self.values[index];
        (index, value)
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
        let mut map = UnorderedMap::new();
        let first = map.add('a');

        assert_eq!(map.get(first), (0, &'a'));
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn add_clear() {
        let mut map = UnorderedMap::new();
        let _first = map.add('a');
        map.clear();

        assert_eq!(map.values.len(), 0);
    }

    #[test]
    fn add_twice_get_second() {
        let mut map = UnorderedMap::new();
        let _first = map.add('a');
        let second = map.add('b');

        assert_eq!(map.get(second), (1, &'b'));
        assert_eq!(map.values.len(), 2);
    }

    #[test]
    fn add_twice_get_first() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let _second = map.add('b');

        assert_eq!(map.get(first), (0, &'a'));
        assert_eq!(map.values.len(), 2);
    }

    #[test]
    fn add_remove() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let (index, _value) = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.values.len(), 0);
    }

    #[test]
    #[should_panic]
    fn add_remove_old_key_panic() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        map.remove(first);

        assert_eq!(map.values.len(), 0);
        map.remove(first); // Panic!
    }

    #[test]
    #[should_panic]
    fn add_get_old_key_panic() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        map.remove(first);

        assert_eq!(map.values.len(), 0);
        map.get(first); // Panic!
    }

    #[test]
    fn add_twice_remove_second() {
        let mut map = UnorderedMap::new();
        let _first = map.add('a');
        let second = map.add('b');
        let (index, _value) = map.remove(second);

        assert_eq!(index, 1);
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn add_twice_remove_first() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let _second = map.add('b');
        let (index, _value) = map.remove(first);

        assert_eq!(index, 0);
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn add_twice_remove_first_swaps() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let second = map.add('b');
        map.remove(first);

        assert_eq!(map.get(second), (0, &'b'));
        assert_eq!(map.values.len(), 1);
    }

    #[test]
    fn add_thrice_remove_first_swaps_ignores_second() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let second = map.add('b');
        let _third = map.add('c');
        map.remove(first);

        assert_eq!(map.get(second), (1, &'b'));
        assert_eq!(map.values.len(), 2);
    }

    #[test]
    fn add_twice_remove_first_add() {
        let mut map = UnorderedMap::new();
        let first = map.add('a');
        let _second = map.add('b');
        map.remove(first);
        let third = map.add('c');

        assert_eq!(map.get(third), (1, &'c'));
        assert_eq!(map.values.len(), 2);
    }

    // ////////////////////////////////////////////////////////////////////////////
    // Benches
    // ////////////////////////////////////////////////////////////////////////////

    const ITERATIONS: usize = 1000;

    #[bench]
    fn bench_cycle(bench: &mut Bencher) {
        let mut map = UnorderedMap::new();

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                let a = map.add('a');
                black_box(map.get(a));
                black_box(map.remove(a));
            }
        });
    }

    #[bench]
    fn bench_get(bench: &mut Bencher) {
        let mut map = UnorderedMap::new();
        let a = map.add('a');

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                black_box(map.get(a));
            }
        });
    }

    #[bench]
    fn bench_add(bench: &mut Bencher) {
        let mut map = UnorderedMap::new();

        bench.iter(|| {
            for _ in 0..ITERATIONS {
                black_box(map.add('a'));
            }
        });
    }
}
