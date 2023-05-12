use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

// Result<Command, CargoError>
type TestResult = Result<(), Box<dyn std::error::Error>>;

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
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);

    Ok(())
}

#[test]
// echo -n "Hello  there"
fn hello1_n() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.n.txt")?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(["-n", "Hello  there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
// echo "Hello"  "there"
fn hello2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
// echo -n "Hello"  "there"
fn hello2_n() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.n.txt")?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(["-n", "Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
