use assert_cmd::Command;

#[test]
fn runs() {
    let cmd = Command::cargo_bin("hello");
    let mut res = cmd.unwrap();
    res.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let cmd = Command::cargo_bin("true");
    let mut res = cmd.unwrap();
    res.assert().success();
}

#[test]
fn false_ok() {
    let cmd = Command::cargo_bin("false");
    let mut res = cmd.unwrap();
    res.assert().failure();
}
