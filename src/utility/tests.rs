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

    map.remove(a);
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
    let a = map.add(0);
    let b = map.add(1);
    let c = map.add(2);
    assert_eq!(*map.get(&a), 0);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(map.len(), 3);

    map.remove(a);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(map.len(), 2);

    let d = map.add(3);
    assert_eq!(*map.get(&b), 1);
    assert_eq!(*map.get(&c), 2);
    assert_eq!(*map.get(&d), 3);
    assert_eq!(map.len(), 3);
}
