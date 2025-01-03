use assert_cmd::Command;
use std::path::PathBuf;

#[test]
fn test_cli_with_small_image(){
    let mut cmd = Command::cargo_bin("ansi-gen").unwrap();
    let test_image_path = PathBuf::from("tests/fixtures/rust-crab.png");

    cmd.arg(test_image_path)
        .arg("4");

    cmd.assert().success();
}

