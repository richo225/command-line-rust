use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(expected_file: &str, args: &[&str]) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn dies_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("<FILE_NAME>"));

    Ok(())
}

#[test]
fn empty() -> TestResult {
    run("tests/expected/empty.txt.out", &["tests/inputs/empty.txt"])
}

#[test]
fn empty_b() -> TestResult {
    run(
        "tests/expected/empty.b.txt.out",
        &["tests/inputs/empty.b.txt"],
    )
}

#[test]
fn empty_n() -> TestResult {
    run(
        "tests/expected/empty.n.txt.out",
        &["tests/inputs/empty.n.txt"],
    )
}
