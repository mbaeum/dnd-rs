use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

pub fn get_cli_command(folder_path: Option<&str>, env: Option<&str>) -> Command {
    let mut cmd = Command::cargo_bin("cli").unwrap();

    let default_path = "../.config";

    cmd.env("DND_SETTINGS_DIR", folder_path.unwrap_or(default_path));

    match env {
        Some(s) => {
            cmd.env("ENV", s);
        }
        None => {}
    };

    cmd
}

#[derive(Debug)]
pub enum CommandAssertionFailure {
    FailDidNotFail,
    FailNotInStdErr,
    SuccessNotInStdOut,
    SuccessDidNotSucceeed,
}

pub fn assert_failure_contains(
    mut cmd: Command,
    pattern: &str,
) -> Result<(), CommandAssertionFailure> {
    match cmd.assert().try_failure() {
        Ok(a) => match a.try_stderr(predicate::str::contains(pattern)) {
            Ok(_) => Ok(()),
            Err(_) => Err(CommandAssertionFailure::FailNotInStdErr),
        },
        Err(_) => Err(CommandAssertionFailure::FailDidNotFail),
    }
}

pub fn assert_success_contains(
    mut cmd: Command,
    pattern: &str,
) -> Result<(), CommandAssertionFailure> {
    match cmd.assert().try_success() {
        Ok(a) => match a.try_stdout(predicate::str::contains(pattern)) {
            Ok(_) => Ok(()),
            Err(_) => Err(CommandAssertionFailure::SuccessNotInStdOut),
        },
        Err(_) => Err(CommandAssertionFailure::SuccessDidNotSucceeed),
    }
}
