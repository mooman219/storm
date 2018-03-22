#![feature(test)]

extern crate storm;
extern crate test;

use storm::utility::single_spsc;

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
