use std::process::Command;

#[test]
fn prints_message_with_newline() {
    let output = Command::new(env!("CARGO_BIN_EXE_echor"))
        .args(["-m", "hello", "world"])
        .output()
        .expect("failed to execute");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello world\n");
}

#[test]
fn prints_message_without_newline() {
    let output = Command::new(env!("CARGO_BIN_EXE_echor"))
        .args(["-m", "hello", "world", "-n"])
        .output()
        .expect("failed to execute");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello world");
}

#[test]
fn fail_when_no_message_provided() {
    let output = Command::new(env!("CARGO_BIN_EXE_echor"))
        .output()
        .expect("failed to execute");

    assert!(!output.status.success());
}
