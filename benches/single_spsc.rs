#![feature(test)]

extern crate bounded_spsc_queue;
extern crate storm;
extern crate test;

use std::sync::mpsc::sync_channel;
use storm::utility::benching::black_box;
use storm::utility::single_spsc;
use test::Bencher;

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
