use std::fs::{self};

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// const INACCESSIBLE: &str = "tests/inputs/inaccessible.txt";
const NONEXISTENT: &str = "tests/inputs/thisfiledoesnotexist.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

fn run_stdout(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

fn run_stderr(args: &[&str], expected_predicate: &str) -> TestResult {
    let mut cmd = Command::cargo_bin("catr")?;

    cmd.args(args)
        .assert()
        .success()
        .stderr(predicate::str::contains(expected_predicate));

    Ok(())
}

#[test]
fn errors_with_nonexistent_file() -> TestResult {
    run_stderr(&[NONEXISTENT], "No such file or directory (os error 2)")
}

// #[test]
// fn errors_with_inaccessible_file() -> TestResult {
//     // let mut permissions = File::open(INACCESSIBLE)
//     //     .unwrap()
//     //     .metadata()
//     //     .unwrap()
//     //     .permissions();

//     // permissions.set_mode(000);

//     run_stderr(&[INACCESSIBLE], "Permission denied (os error 13)")
// }

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin("catr")?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run_stdout(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_b() -> TestResult {
    run_stdout(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn empty_n() -> TestResult {
    run_stdout(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn fox() -> TestResult {
    run_stdout(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> TestResult {
    run_stdout(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> TestResult {
    run_stdout(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> TestResult {
    run_stdout(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run_stdout(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> TestResult {
    run_stdout(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

#[test]
fn bustle() -> TestResult {
    run_stdout(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    run_stdout(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> TestResult {
    run_stdout(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

#[test]
fn all() -> TestResult {
    run_stdout(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run_stdout(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> TestResult {
    run_stdout(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
