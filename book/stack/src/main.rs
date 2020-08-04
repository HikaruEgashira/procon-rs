fn main() {
    let mut s = vec![];
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(Some(&3), s.last());
    s.pop();
    assert_eq!(Some(&2), s.last());
    s.pop();
    assert_eq!(Some(&1), s.last());
    s.pop();
    assert_eq!(None, s.last());
}
