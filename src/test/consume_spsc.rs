use utility::consume_spsc;

#[test]
fn test_consume_spsc() {
    let (producer, consumer) = consume_spsc::make();

    assert_eq!(consumer.consume(), None);

    producer.set(0u32);
    assert_eq!(consumer.consume(), Some(0u32));
    assert_eq!(consumer.consume(), None);

    producer.set(0u32);
    producer.set(1u32);
    assert_eq!(consumer.consume(), Some(1u32));
    assert_eq!(consumer.consume(), None);
}
