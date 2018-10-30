use utility::indexmap::*;

#[test]
fn test_indexmap_misc() {
    let mut map = IndexMap::new();
    let a = map.add();
    let b = map.add();
    let c = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.get(b), 1);
    assert_eq!(map.get(c), 2);
    assert_eq!(map.len(), 3);

    assert_eq!(map.remove(a), 0);
    assert_eq!(map.get(b), 1);
    assert_eq!(map.get(c), 0);
    assert_eq!(map.len(), 2);

    let d = map.add();
    assert_eq!(map.get(b), 1);
    assert_eq!(map.get(c), 0);
    assert_eq!(map.get(d), 2);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_indexmap_remove() {
    let mut map = IndexMap::new();
    let a = map.add();
    let b = map.add();
    let c = map.add();
    let d = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.get(b), 1);
    assert_eq!(map.get(c), 2);
    assert_eq!(map.get(d), 3);
    assert_eq!(map.len(), 4);

    assert_eq!(map.remove(d), 3);
    assert_eq!(map.remove(a), 0);
    assert_eq!(map.get(c), 0);
}

#[test]
fn test_indexmap_cycle() {
    let mut map = IndexMap::new();
    let a = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.remove(a), 0);
    let a = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.remove(a), 0);

    let a = map.add();
    let b = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.get(b), 1);
    assert_eq!(map.remove(a), 0);
    assert_eq!(map.remove(b), 0);

    let a = map.add();
    let b = map.add();
    assert_eq!(map.get(a), 0);
    assert_eq!(map.get(b), 1);
    assert_eq!(map.remove(a), 0);
    assert_eq!(map.remove(b), 0);
}
