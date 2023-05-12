use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

// Result<Command, CargoError>
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run_command(expected_file: &str, args: &[&str]) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn dies_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided:",
    ));

    Ok(())
}

#[test]
fn runs_with_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("here is some test text").assert().success();

    Ok(())
}

#[test]
// echo "Hello there"
fn hello1() -> TestResult {
    run_command("tests/expected/hello1.txt", &["Hello", "there"])
}

#[test]
// echo -n "Hello  there"
fn hello1_n() -> TestResult {
    run_command("tests/expected/hello1.n.txt", &["-n", "Hello  there"])
}

#[test]
// echo "Hello"  "there"
fn hello2() -> TestResult {
    run_command("tests/expected/hello2.txt", &["Hello", "there"])
}

#[test]
// echo -n "Hello"  "there"
fn hello2_n() -> TestResult {
    run_command("tests/expected/hello2.n.txt", &["-n", "Hello", "there"])
}
