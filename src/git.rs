use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::readme;

const BINARY: &str = "git";

pub fn commit_and_push() -> Result<()> {
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
        .arg(readme::FILE_NAME)
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
