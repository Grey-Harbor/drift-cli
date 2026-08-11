use predicates::prelude::*;

#[test]
fn json_mode_wraps_parse_errors() {
    let mut command = assert_cmd::cargo::cargo_bin_cmd!("drift");
    command
        .args(["--json", "key", "create", "--label", "reporting"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(
            predicate::str::contains("\"schemaVersion\": 1")
                .and(predicate::str::contains("\"kind\": \"usage\""))
                .and(predicate::str::contains("--scope")),
        );
}
