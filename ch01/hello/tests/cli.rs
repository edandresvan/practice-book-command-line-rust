#[test]
fn works() {
  assert!(true);
}

#[test]
fn runs_1() {
  let mut cmd: std::process::Command = std::process::Command::new("ls");
  let res: Result<std::process::Output, std::io::Error> = cmd.output();
  assert!(res.is_ok());
}

#[test]
fn runs_2() {
  let mut cmd: assert_cmd::Command = assert_cmd::Command::cargo_bin("hello").unwrap();
  cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn runs_ok() {
  let mut cmd: assert_cmd::Command = assert_cmd::Command::cargo_bin("true").unwrap();
  cmd.assert().success();
}

#[test]
fn runs_not_ok() {
  let mut cmd: assert_cmd::Command = assert_cmd::Command::cargo_bin("false").unwrap();
  cmd.assert().failure();
}
