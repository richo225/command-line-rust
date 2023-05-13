use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("<FILE_NAME>"));

    Ok(())
}
