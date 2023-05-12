use assert_cmd::Command;
use predicates::prelude::predicate;

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
