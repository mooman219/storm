use bounded_spsc_queue;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Instant;
use test::Bencher;
use time::convert::*;
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

    assert_eq!(consumer.pop(), None);

    producer.push(0u32);
    assert_eq!(consumer.pop(), Some(0u32));
    assert_eq!(consumer.pop(), None);

    producer.push(0u32);
    producer.push(1u32);
    assert_eq!(consumer.pop(), Some(1u32));
    assert_eq!(consumer.pop(), None);
}

#[bench]
fn bench_single_spsc_cycle(bench: &mut Bencher) {
    let (producer, consumer) = single_spsc::make();

    bench.iter(|| {
        producer.push(0u32);
        black_box(consumer.pop());
    });
}

pub fn compare_single_spsc_throughput() {
    let iterations: i64 = 20000i64;

    //
    // Bounded SPSC
    //
    {
        let (p, c) = bounded_spsc_queue::make(iterations as usize);
        let start = Instant::now();
        for i in 0..iterations as usize {
            black_box(p.push(i));
            black_box(c.pop());
        }
        let duration = as_nanoseconds(&start.elapsed());
        let throughput = iterations as f64 / (duration as f64) * 1000000000f64;
        println!(
            "Bounded SPSC Throughput     : {:14.2}/s -- (iterations: {} in {:7} ns)",
            throughput, iterations, duration
        );
    }
    //
    // Single SPSC
    //
    {
        let (p, c) = single_spsc::make();
        let start = Instant::now();
        for i in 0..iterations as usize {
            black_box(p.push(i));
            black_box(c.try_pop());
        }
        let duration = as_nanoseconds(&start.elapsed());
        let throughput = iterations as f64 / (duration as f64) * 1000000000f64;
        println!(
            "Single SPSC Throughput      : {:14.2}/s -- (iterations: {} in {:7} ns)",
            throughput, iterations, duration
        );
    }
    //
    // MPSC Sync Channel
    //
    {
        let (tx, rx) = sync_channel(iterations as usize);
        let start = Instant::now();
        for i in 0..iterations as usize {
            black_box(tx.send(i).unwrap());
            black_box(rx.recv().unwrap());
        }
        let duration = as_nanoseconds(&start.elapsed());
        let throughput = iterations as f64 / (duration as f64) * 1000000000f64;
        println!(
            "MPSC Sync Channel Throughput: {:14.2}/s -- (iterations: {} in {:7} ns)",
            throughput, iterations, duration
        );
    }
}
