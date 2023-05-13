use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

fn run(args: &[&str], expected_file: &str) -> TestResult {
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
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.b.txt.out")
}

#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.n.txt.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> TestResult {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
