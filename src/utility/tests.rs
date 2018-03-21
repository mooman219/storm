use bounded_spsc_queue;
use std::sync::mpsc::sync_channel;
use test::Bencher;
use utility::benching::black_box;
use utility::single_spsc;
use utility::slotmap::*;

// ////////////////////////////////////////////////////////
// slotmap
// ////////////////////////////////////////////////////////

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

    bench.iter(|| {
        black_box(map.get(&a));
    });
}

// ////////////////////////////////////////////////////////
// single_spsc
// ////////////////////////////////////////////////////////

#[test]
fn test_single_spsc() {
    let (producer, consumer) = single_spsc::make();

    assert_eq!(consumer.try_pop(), None);

    producer.push(0u32);
    assert_eq!(consumer.try_pop(), Some(0u32));
    assert_eq!(consumer.try_pop(), None);

    producer.push(0u32);
    producer.push(1u32);
    assert_eq!(consumer.try_pop(), Some(1u32));
    assert_eq!(consumer.try_pop(), None);
}

#[bench]
fn bench_single_spsc_self_cycle(bench: &mut Bencher) {
    let (p, c) = single_spsc::make();

    bench.iter(|| {
        black_box(p.push(1));
        black_box(c.try_pop());
    });
}

#[bench]
fn bench_single_spsc_bounded_spsc_cycle(bench: &mut Bencher) {
    let (p, c) = bounded_spsc_queue::make(10000 as usize);

    bench.iter(|| {
        black_box(p.push(1));
        black_box(c.pop());
    });
}

#[bench]
fn bench_single_spsc_sync_mpsc_cycle(bench: &mut Bencher) {
    let (tx, rx) = sync_channel(10000 as usize);

    bench.iter(|| {
        black_box(tx.send(1).unwrap());
        black_box(rx.recv().unwrap());
    });
}