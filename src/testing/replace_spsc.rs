use utility::replace_spsc;

#[test]
fn test_replace_spsc() {
    let (producer, consumer) = replace_spsc::make(1u32);

    assert_eq!(consumer.get(), 1u32);

    producer.set(2u32);
    assert_eq!(consumer.get(), 2u32);
    assert_eq!(consumer.get(), 2u32);

    producer.set(3u32);
    producer.set(4u32);
    assert_eq!(consumer.get(), 4u32);
    assert_eq!(consumer.get(), 4u32);
}
