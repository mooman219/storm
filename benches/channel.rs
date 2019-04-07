#![feature(test, asm)]
#![allow(dead_code)]

extern crate cgmath;
extern crate core;
extern crate parking_lot;
extern crate storm;
extern crate test;

mod utility;

use std::sync::mpsc::sync_channel;
use test::Bencher;
use utility::benching::black_box;
use utility::bounded_spsc;
use utility::consume_spsc;
use utility::replace_spsc;

const ITERATIONS: usize = 1000;

#[bench]
fn bench_channel_consume_cycle(bench: &mut Bencher) {
    let (p, c) = consume_spsc::make();

    bench.iter(|| {
        for x in 0..ITERATIONS {
            black_box(p.set(x));
            black_box(c.consume());
        }
    });
}

#[bench]
fn bench_channel_replace_cycle(bench: &mut Bencher) {
    let (p, c) = replace_spsc::make(1);

    bench.iter(|| {
        for x in 0..ITERATIONS {
            black_box(p.set(x));
            black_box(c.get());
        }
    });
}

#[bench]
fn bench_channel_bounded_cycle(bench: &mut Bencher) {
    let (p, c) = bounded_spsc::make(10000 as usize);

    bench.iter(|| {
        for x in 0..ITERATIONS {
            black_box(p.push(x));
            black_box(c.pop());
        }
    });
}

#[bench]
fn bench_channel_sync_mpsc_cycle(bench: &mut Bencher) {
    let (tx, rx) = sync_channel(10000 as usize);

    bench.iter(|| {
        for x in 0..ITERATIONS {
            black_box(tx.send(x).unwrap());
            black_box(rx.recv().unwrap());
        }
    });
}
