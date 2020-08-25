use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"8
3 3 4 4 5 8 7 9
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9 7 8 5 4 4 3 3\n");
    assert!(output.stderr_str().is_empty());
}

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
