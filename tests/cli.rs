use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("vibinremote")?;

    cmd.assert().failure().stderr(predicate::str::contains("were not provided"));

    Ok(())
}

#[test]
fn config_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("vibinremote")?;
    cmd.arg("--config").arg("does/not/exist.json");

    cmd.assert().failure().stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn config_file_contains_invalid_json() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = assert_fs::NamedTempFile::new("test.json")?;
    json_file.write_str("this is not JSON")?;

    let mut cmd = Command::cargo_bin("vibinremote")?;
    cmd.arg("--config").arg(json_file.path());

    cmd.assert().failure().stderr(predicate::str::contains("Could not parse config data"));

    Ok(())
}