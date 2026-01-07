use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs_with_basic_file() {
    let mut cmd = Command::cargo_bin("catr").unwrap();
    cmd.arg("tests/inputs/fox.txt")
       .assert()
       .success()
       .stdout(predicate::str::contains("fox"));
}

#[test]
fn test_number_lines_flag() {
    let mut cmd = Command::cargo_bin("catr").unwrap();
    cmd.args(["-n", "tests/inputs/fox.txt"])
       .assert()
       .success()
       .stdout(predicate::str::starts_with("     1\tThe quick brown fox"));
}
