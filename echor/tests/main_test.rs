use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

#[test]
fn dies_with_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided:",
    ));
}

#[test]
fn runs_with_arg() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("here is some test text").assert().success();
}

#[test]
// echo "Hello there"
fn hello1() {
    let expected = fs::read_to_string("tests/expected/hello1.txt").unwrap();

    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
// echo -n "Hello  there"
fn hello1_n() {
    let expected = fs::read_to_string("tests/expected/hello1.n.txt").unwrap();

    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["-n", "Hello  there"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
// echo "Hello"  "there"
fn hello2() {
    let expected = fs::read_to_string("tests/expected/hello2.txt").unwrap();

    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
}

#[test]
// echo -n "Hello"  "there"
fn hello2_n() {
    let expected = fs::read_to_string("tests/expected/hello2.n.txt").unwrap();

    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(["-n", "Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
}
