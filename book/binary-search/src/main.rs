fn my_binary_search(vec: Vec<i32>, target: i32) -> Option<usize> {
    let mut front = 0;
    let mut tail = vec.len();
    let mut res: Option<usize> = None;
    while tail - front > 0 {
        let index = (front + tail) / 2;
        let center = vec[index];
        if center == target {
            res = Some(index);
            break;
        } else if center > target {
            tail = index;
        } else {
            front = index;
        }
    }
    res
}

fn main() {
    let vec = vec![-5, 0, 1, 3, 4, 5, 5, 8, 10, 13];
    let index = vec.binary_search(&1).unwrap();
    assert_eq!(index, 2);

    let index = my_binary_search(vec, 10).unwrap();
    assert_eq!(index, 8);
}
