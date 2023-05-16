use assert_cmd::Command;
use predicates::prelude::*;
// use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const NONEXISTENT: &str = "tests/inputs/thisfiledoesnotexist.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

// fn run_stdout(args: &[&str], expected_file: &str) -> TestResult {
//     let expected = fs::read_to_string(expected_file)?;

//     let mut cmd = Command::cargo_bin("catr")?;
//     cmd.args(args).assert().success().stdout(expected);

//     Ok(())
// }

// fn run_stderr(args: &[&str], expected_predicate: &str) -> TestResult {
//     let mut cmd = Command::cargo_bin("catr")?;

//     cmd.args(args)
//         .assert()
//         .success()
//         .stderr(predicate::str::contains(expected_predicate));

//     Ok(())
// }

// #[test]
// fn errors_with_nonexistent_file() -> TestResult {
//     run_stderr(&[NONEXISTENT], "No such file or directory (os error 2)")
// }
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin("headr")?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn test_parse_string_to_int() {
    let actual = headr::parse_string_to_int("3");
    let expected = 3;
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), expected);

    let actual = headr::parse_string_to_int("hello");
    let expected = "hello".to_string();
    assert!(actual.is_err());
    assert_eq!(actual.unwrap_err().to_string(), expected);

    let actual = headr::parse_string_to_int("0");
    let expected = "0".to_string();
    assert!(actual.is_err());
    assert_eq!(actual.unwrap_err().to_string(), expected);
}
