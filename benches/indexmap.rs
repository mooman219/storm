#![feature(test)]

extern crate storm;
extern crate test;

use storm::utility::benching::black_box;
use storm::utility::indexmap::*;
use test::Bencher;

#[bench]
fn bench_indexmap_cycle(bench: &mut Bencher) {
    let mut map = IndexMap::new();

    bench.iter(|| {
        let a = map.add();
        black_box(map.get(&a));
        black_box(map.remove(&a));
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
