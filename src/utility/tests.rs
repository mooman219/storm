use test::Bencher;
use utility::benching::black_box;
use utility::slotmap::*;

#[test]
fn test_indexmap() {
    let mut map = IndexMap::new();
    let a = map.add();
    let b = map.add();
    let c = map.add();
    assert_eq!(map.get(&a), 0);
    assert_eq!(map.get(&b), 1);
    assert_eq!(map.get(&c), 2);
    assert_eq!(map.len(), 3);

    assert_eq!(map.remove(a), 0);
    assert_eq!(map.get(&b), 1);
    assert_eq!(map.get(&c), 0);
    assert_eq!(map.len(), 2);

    let d = map.add();
    assert_eq!(map.get(&b), 1);
    assert_eq!(map.get(&c), 0);
    assert_eq!(map.get(&d), 2);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_slotmap() {
    let mut map = SlotMap::new();
    let a = map.add(9);
    let b = map.add(1);
    let c = map.add(2);
    assert_eq!(*map.get(&a), 9);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(map.len(), 3);

    assert_eq!(map.remove(a), 9);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(map.len(), 2);

    let d = map.add(3);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(*map.get(&d), 3);
    assert_eq!(map.len(), 3);
}

#[bench]
fn bench_indexmap_cycle(bench: &mut Bencher) {
    let mut map = IndexMap::new();

    bench.iter(|| {
        let a = map.add();
        black_box(map.get(&a));
        black_box(map.remove(a));
    });
}

#[bench]
fn bench_indexmap_get(bench: &mut Bencher) {
    let mut map = IndexMap::new();
    let a = map.add();
    for _ in 0..20 {
        map.add();
    }
    let b = map.add();
    for _ in 0..20 {
        map.add();
    }
    let c = map.add();
    for _ in 0..20 {
        map.add();
    }
    let d = map.add();

    bench.iter(|| {
        for _ in 0..20 {
            black_box(map.get(&a));
            black_box(map.get(&b));
            black_box(map.get(&c));
            black_box(map.get(&d));
        }
    });
}
