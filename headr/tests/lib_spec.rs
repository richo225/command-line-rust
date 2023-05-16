use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs::File, io::Read};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const NONEXISTENT: &str = "tests/inputs/thisfiledoesnotexist.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

fn run_stdout(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    let mut cmd = Command::cargo_bin("headr")?;
    cmd.args(args)
        .assert()
        .success()
        .stdout(predicate::eq(expected.as_bytes() as &[u8]));

    Ok(())
}

fn run_stderr(args: &[&str], expected_predicate: &str) -> TestResult {
    let mut cmd = Command::cargo_bin("headr")?;

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
fn empty() -> TestResult {
    run_stdout(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n2() -> TestResult {
    run_stdout(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

// --------------------------------------------------
#[test]
fn empty_n4() -> TestResult {
    run_stdout(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

// --------------------------------------------------
#[test]
fn empty_c2() -> TestResult {
    run_stdout(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

// --------------------------------------------------
#[test]
fn empty_c4() -> TestResult {
    run_stdout(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn one() -> TestResult {
    run_stdout(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> TestResult {
    run_stdout(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run_stdout(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> TestResult {
    run_stdout(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run_stdout(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run_stdout(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn two() -> TestResult {
    run_stdout(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> TestResult {
    run_stdout(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run_stdout(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> TestResult {
    run_stdout(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> TestResult {
    run_stdout(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn three() -> TestResult {
    run_stdout(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> TestResult {
    run_stdout(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run_stdout(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> TestResult {
    run_stdout(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> TestResult {
    run_stdout(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn ten() -> TestResult {
    run_stdout(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> TestResult {
    run_stdout(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run_stdout(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2() -> TestResult {
    run_stdout(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run_stdout(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn multiple_files() -> TestResult {
    run_stdout(&[EMPTY, ONE, TWO, THREE, TEN], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> TestResult {
    run_stdout(
        &[EMPTY, ONE, TWO, THREE, TEN, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> TestResult {
    run_stdout(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> TestResult {
    run_stdout(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> TestResult {
    run_stdout(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> TestResult {
    run_stdout(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.c4.out",
    )
}
