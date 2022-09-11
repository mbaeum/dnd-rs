mod common;

#[test]
fn test_single_random_spell() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("spell").args(&["-r"]);
    common::assert_success_contains(cmd, "Classes:\n").unwrap();

    Ok(())
}

#[test]
fn test_spell_by_class() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("spell").args(&["-r", "-c", "bard"]);
    common::assert_success_contains(cmd, "Bard").unwrap();

    Ok(())
}

#[test]
fn test_spell_by_level() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("spell").args(&["-r", "-l", "2"]);
    let pattern = r"Level:[\n,\r,\s,\t]+[1,2]";
    common::assert_success_contains(cmd, pattern).unwrap();

    Ok(())
}

#[test]
fn test_spell_by_exact_level() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("spell").args(&["-r", "-l", "2", "-e"]);
    common::assert_success_contains(cmd, "Level:\n\t2").unwrap();

    Ok(())
}

#[test]
fn test_spell_with_bad_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("spell").args(&["not", "a", "set of", "commands"]);
    common::assert_failure_contains(cmd, "USAGE").unwrap();

    Ok(())
}
