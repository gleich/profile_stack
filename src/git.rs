use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::conf::Env;

const BINARY: &str = "git";

pub fn commit_and_push(env_var_conf: &Env) -> Result<()> {
    Command::new(BINARY)
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg("action@github.com")
        .output()
        .context("Failed to set commit email")?;
    Command::new(BINARY)
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg("Publishing Bot")
        .output()
        .context("Failed to set commit name")?;
    Command::new(BINARY)
        .arg("add")
        .arg(&env_var_conf.output_file)
        .output()
        .context("Failed to stage changes")?;
    Command::new(BINARY)
        .arg("commit")
        .arg("-m")
        .arg("Update profile stack")
        .output()
        .context("Failed to commit staged changes")?;
    Command::new(BINARY)
        .arg("push")
        .output()
        .context("Failed to push committed changes")?;
    Ok(())
}

pub fn repo_owner() -> Result<String> {
    let remote = Command::new(BINARY)
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .stdout(Stdio::piped())
        .output()
        .context("Failed to get remote URL")?;
    Ok(String::from_utf8(remote.stdout)?
        .split("/")
        .collect::<Vec<&str>>()
        .get(3)
        .unwrap()
        .to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn test_repo_owner() -> Result<()> {
        assert_eq!(String::from("gleich"), super::repo_owner()?);
        Ok(())
    }
}
