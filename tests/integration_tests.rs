use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn run_single_test_success() {
    Command::cargo_bin("ucan_config")
        .unwrap()
        .arg("-h")
        .assert()
        .success();
}