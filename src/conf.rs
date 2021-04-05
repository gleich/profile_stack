use std::env;
use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug)]
pub struct Env {
	pub path: PathBuf,
	pub badges: bool,
	pub technology_emoji: char,
	pub project_emoji: char,
}

impl Env {
	fn get_env_var(name: &str, default: &str) -> Result<String, anyhow::Error> {
		let value = env::var(format!("INPUTS_{}", name.to_uppercase()));
		Ok(value.unwrap_or(default.to_string()))
	}

	pub fn get() -> Result<Env, anyhow::Error> {
		Ok(Env {
			path: Path::new(Env::get_env_var("path", "stack.yml")?.as_str()).to_owned(),
			badges: Env::get_env_var("badges", "true")?.parse()?,
			technology_emoji: Env::get_env_var("technology_emoji", "💻")?.parse()?,
			project_emoji: Env::get_env_var("project_emoji", "🚀")?.parse()?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_env_get() -> Result<(), anyhow::Error> {
		let env_var_conf = Env::get()?;
		assert_eq!(env_var_conf.path, Path::new("stack.yml"));
		assert_eq!(env_var_conf.badges, true);
		assert_eq!(env_var_conf.technology_emoji, '💻');
		assert_eq!(env_var_conf.project_emoji, '🚀');
		Ok(())
	}
}
