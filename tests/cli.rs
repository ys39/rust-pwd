use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::env;
use std::process::Command; // Run programs

#[test]
fn invalid_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-pwd")?;

    cmd.arg("-a");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-a' found",
    ));
    Ok(())
}

#[test]
fn help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-pwd")?;

    cmd.arg("--help");
    let contains_predicate = predicate::str::contains(
        "-P, --physical  -P, --physical avoid all symlinks",
    )
    .and(predicate::str::contains(
        "-L, --logical   -L, --logical use PWD from environment, even if it contains symlinks",
    ))
    .and(predicate::str::contains("-h, --help      Print help"))
    .and(predicate::str::contains("-V, --version   Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_logical_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-pwd")?;
    // PWD を一時的に設定
    env::set_var("PWD", "/tmp/some/path");

    cmd.arg("-L");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/tmp/some/path"));
    Ok(())
}

#[test]
fn long_logical_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-pwd")?;
    // PWD を一時的に設定
    env::set_var("PWD", "/tmp/some/path");

    cmd.arg("--logical");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/tmp/some/path"));
    Ok(())
}
