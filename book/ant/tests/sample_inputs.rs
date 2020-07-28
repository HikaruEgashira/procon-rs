use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10
3
2 6 7
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"min = 4
max = 8
"#
    );
    assert!(output.stderr_str().is_empty());
}
