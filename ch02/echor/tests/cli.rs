use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const BIN_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn dies_no_args() -> TestResult {
  let mut cmd = Command::cargo_bin(BIN_NAME)?;
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Usage"));
  Ok(())
}

#[test]
fn hello1() -> TestResult {
  let outfile: &str = "tests/expected/hello1.txt";
  let expected: String = fs::read_to_string(outfile)?;

  let mut cmd: Command = Command::cargo_bin(BIN_NAME)?;
  cmd.arg("Hello there").assert().success().stdout(expected);
  Ok(())
}

#[test]
fn hello2() -> TestResult {
  let expected: String = fs::read_to_string("tests/expected/hello2.txt")?;
  let mut cmd: Command = Command::cargo_bin(BIN_NAME)?;
  cmd
    .args(vec!["Hello", "there"])
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}

fn run(
  args: &[&str],
  expected_file: &str,
) -> TestResult {
  let expected: String = fs::read_to_string(expected_file)?;
  Command::cargo_bin(BIN_NAME)?
    .args(args)
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}

#[test]
fn hello1b() -> TestResult {
  run(&["Hello", "there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2b() -> TestResult {
  run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1b_no_newline() -> TestResult {
  run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2b_no_newline() -> TestResult {
  run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
