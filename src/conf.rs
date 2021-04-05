use std::env;

use anyhow::Result;

#[derive(Debug)]
pub struct Env {
	pub path: String,
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
			path: Env::get_env_var("path", "stack.yml")?,
			badges: Env::get_env_var("badges", "true")?.parse()?,
			technology_emoji: Env::get_env_var("technology_emoji", "💻")?.parse()?,
			project_emoji: Env::get_env_var("project_emoji", "🚀")?.parse()?,
		})
	}
}
