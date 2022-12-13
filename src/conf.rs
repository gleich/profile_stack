use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::Context;
use serde::Deserialize;

#[derive(PartialEq, Debug)]
pub struct Env {
    pub badges: bool,
    pub technology_emoji: char,
    pub project_emoji: char,
    pub output_file: PathBuf,
    pub config_filename: PathBuf,
}

fn get_env_var(name: &str, default: &str) -> Result<String, anyhow::Error> {
    let value = env::var(format!("INPUT_{}", name.to_uppercase()));
    Ok(value.unwrap_or(default.to_string()))
}

pub fn env_vars() -> Result<Env, anyhow::Error> {
    Ok(Env {
        badges: get_env_var("badges", "true")?.parse()?,
        technology_emoji: get_env_var("technology_emoji", "💻")?.parse()?,
        project_emoji: get_env_var("project_emoji", "🚀")?.parse()?,
        config_filename: Path::new(&get_env_var("path", "stack.yml")?).to_owned(),
        output_file: Path::new(&get_env_var("output_file", "README.md")?).to_owned(),
    })
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct Project {
    pub url: String,
    #[serde(default = "bool::default")]
    pub wip: bool,
}

fn default_color() -> String { String::from("#FFFFFF") }
#[derive(PartialEq, Deserialize, Debug)]
pub struct Technology {
    pub name: String,
    pub logo: String,
    #[serde(default = "default_color")]
    pub logo_color: String,
    pub url: String,
    pub color: String,
    pub projects: Vec<Project>,
}

pub fn config_file(env_conf: &Env) -> Result<Vec<Technology>, anyhow::Error> {
    let content = fs::read_to_string(&env_conf.config_filename)?;
    let deserialized: Vec<Technology> =
        serde_yaml::from_str(&content).context("Deserialize failed")?;
    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use super::*;

    #[test]
    fn test_config_file() -> Result<(), anyhow::Error> {
        // Creating a test file
        let tmp_dir = "tests";
        fs::create_dir(tmp_dir)?;
        let config_path = "tests/tmp.yml";
        let readme_path = "content/STACK.md";
        let mut file = File::create(config_path)?;
        file.write_all(
            b"- name: Golang
  logo: go
  url: https://golang.org/
  logo_color: \"#201020\"
  color: \"#7FD6EA\"
  projects:
    - url: https://github.com/gleich/fgh

- name: Python
  logo: python
  url: https://www.python.org/
  color: \"#3C78A9\"
  projects:
    - url: https://github.com/gleich/profile_stack
    - url: https://github.com/gleich/test
      wip: true",
        )?;

        // Getting config data
        let file_conf = config_file(&Env {
            config_filename: Path::new(config_path).to_owned(),
            badges: true,
            technology_emoji: ' ',
            project_emoji: ' ',
            output_file: Path::new(readme_path).to_owned(),
        })?;
        fs::remove_dir_all(tmp_dir)?;

        assert_eq!(
            file_conf,
            vec![
                Technology {
                    name: String::from("Golang"),
                    logo_color: String::from("#201020"),
                    logo: String::from("go"),
                    url: String::from("https://golang.org/"),
                    color: String::from("#7FD6EA"),
                    projects: vec![Project {
                        url: String::from("https://github.com/gleich/fgh"),
                        wip: false
                    },]
                },
                Technology {
                    name: String::from("Python"),
                    logo: String::from("python"),
                    logo_color: String::from("#FFFFFF"),
                    url: String::from("https://www.python.org/"),
                    color: String::from("#3C78A9"),
                    projects: vec![
                        Project {
                            url: String::from("https://github.com/gleich/profile_stack"),
                            wip: false
                        },
                        Project {
                            url: String::from("https://github.com/gleich/test"),
                            wip: true
                        }
                    ]
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_env_vars() -> Result<(), anyhow::Error> {
        assert_eq!(
            env_vars()?,
            Env {
                config_filename: Path::new("stack.yml").to_owned(),
                badges: true,
                technology_emoji: '💻',
                project_emoji: '🚀',
                output_file: Path::new("README.md").to_owned()
            }
        );
        Ok(())
    }
}
