use crate::common::CommandCraterExt;
use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn test_good_config() {
    Command::crater()
        .args(["check-config", "tests/check_config/good.toml"])
        .assert()
        .success();
}

#[test]
fn test_bad_config_duplicate_crate() {
    Command::crater()
        .args([
            "check-config",
            "tests/check_config/bad-duplicate-crate.toml",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("duplicate key `lazy_static` in table `crates`"));
}

#[test]
fn test_bad_config_duplicate_repo() {
    Command::crater()
        .args(["check-config", "tests/check_config/bad-duplicate-repo.toml"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "duplicate key `brson/hello-rs` in table `github-repos`",
        ));
}

#[test]
fn test_bad_config_missing_crate() {
    Command::crater()
        .args(["check-config", "tests/check_config/bad-missing-crate.toml"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("crate `crater_missing_crate` is not available"));
}

#[test]
fn test_bad_config_missing_repo() {
    Command::crater()
        .args(["check-config", "tests/check_config/bad-missing-repo.toml"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("GitHub repo `ghost/missing-repo` is missing"));
}
