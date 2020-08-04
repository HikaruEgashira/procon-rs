use std::collections::VecDeque;

fn main() {
    let mut que = VecDeque::new();
    que.push_back(1); // [1]
    que.push_back(2); // [1, 2]
    que.push_back(3); // [1, 2, 3]
    assert_eq!(que.front(), Some(&1));
    que.pop_front();
    assert_eq!(que.front(), Some(&2));
    que.pop_front();
    assert_eq!(que.front(), Some(&3));
    que.pop_front();
    assert_eq!(que.front(), None);
}
