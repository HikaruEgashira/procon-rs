#[test]
pub fn map_example() {
    let v: Vec<i32> = vec![1, 2, 3]
        .into_iter()
        .map(|x| x + 1)
        .map(|x| x - 2)
        .rev()
        .collect();

    assert_eq!(v, [2, 1, 0]);
}

#[test]
fn collect_example() {
    let a = [1, 2, 3];
    let doubled = a.iter().map(|&x| x * 2).collect::<Vec<i32>>();

    assert_eq!(vec![2, 4, 6], doubled);

    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
    let one_to_one_hundred_2 = one_to_one_hundred
        .iter()
        .map(|&value| value)
        .collect::<Vec<i32>>();

    assert_eq!(one_to_one_hundred, one_to_one_hundred_2);
}
