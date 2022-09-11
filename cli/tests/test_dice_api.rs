mod common;

#[test]
fn test_single_die() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d6+3"]);
    common::assert_success_contains(cmd, "1d6").unwrap();

    Ok(())
}

#[test]
fn test_multi_dice_space() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d6+3 2d3"]);
    common::assert_success_contains(cmd, "1d6").unwrap();

    Ok(())
}
#[test]
fn test_multi_dice_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d6+3", "2d3"]);
    common::assert_success_contains(cmd, "1d6").unwrap();

    Ok(())
}

#[test]
fn test_negative_modifier_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d6-3"]);
    common::assert_failure_contains(cmd, "ParseIntError").unwrap();

    Ok(())
}

#[test]
fn test_negative_count_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["'-1d6+3'"]); //need ticks to avoid reading it as a flag
    common::assert_failure_contains(cmd, "ParseIntError").unwrap();

    Ok(())
}

#[test]
fn test_negative_face_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d-6+3"]);
    common::assert_failure_contains(cmd, "ParseIntError").unwrap();

    Ok(())
}

#[test]
fn test_invalid_separator_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1x6+3"]);
    common::assert_failure_contains(cmd, "InvalidDiceString").unwrap();

    Ok(())
}

#[test]
fn test_multi_dice_comma_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cli_command(None, None);

    cmd.arg("dice").args(&["1d6+3,2d3"]);
    common::assert_failure_contains(cmd, "ParseIntError").unwrap();

    Ok(())
}
