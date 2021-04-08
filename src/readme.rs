use crate::conf;

use anyhow::bail;

pub fn gen_table(
    env_conf: &conf::Env,
    file_conf: &Vec<conf::Technology>,
) -> Result<String, anyhow::Error> {
    let mut lines = Vec::new();

    // Add header
    lines.push(format!(
        "\n| {} **Technology** | {} **Projects** |",
        &env_conf.technology_emoji, &env_conf.project_emoji
    ));
    lines.push(String::from("| - | - |"));

    for tech in file_conf.iter() {
        let mut projects = Vec::new();
        for project in tech.projects.iter() {
            // Getting repo name
            let url_chunks: Vec<&str> = project.url.split('/').collect();
            let repo_name = url_chunks.get(4).unwrap_or(&"").to_owned();
            if repo_name.is_empty() {
                bail!("Failed to extract repository name from {}", project.url)
            }

            // Adding (WIP) to message if wip
            let mut message = String::from(repo_name);
            if project.wip {
                message.push_str("%20(WIP)");
            }

            // Add badge/url
            if env_conf.badges {
                projects.push(format!("![{}](https://img.shields.io/static/v1?label=&message={}&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605)", repo_name, message));
            } else {
                projects.push(format!("[{}]({})", repo_name, message));
            }
        }
        let joined_projects = projects.join(" ");

        if env_conf.badges {
            lines.push(format!("| [![{}](https://img.shields.io/static/v1?label=&message={}&color={}&logo={}&logoColor={})]({}) | {} |",
            tech.name,
            tech.name,
            tech.color.replace("#", ""),
            tech.logo,
            tech.logo_color.replace("#", ""),
            tech.url,
            joined_projects
        ))
        } else {
            lines.push(format!(
                "| [{}]({}) | {} |",
                tech.name, tech.url, joined_projects,
            ));
        }
    }

    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use conf::{Env, Project, Technology};

    use super::*;

    #[test]
    fn test_gen_table() -> Result<(), anyhow::Error> {
        assert_eq!(
            gen_table(
                &Env {
                    path: Path::new("stack.yml").to_owned(),
                    badges: true,
                    technology_emoji: '💻',
                    project_emoji: '🚀',
                },
                &vec![
                    Technology {
                        name: String::from("Golang"),
                        logo_color: String::from("#201020"),
                        logo: String::from("go"),
                        url: String::from("https://golang.org/"),
                        color: String::from("#7FD6EA"),
                        projects: vec![Project {
                            url: String::from("https://github.com/Matt-Gleich/fgh"),
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
                                url: String::from("https://github.com/Matt-Gleich/profile_stack"),
                                wip: false
                            },
                            Project {
                                url: String::from("https://github.com/Matt-Gleich/test"),
                                wip: true
                            }
                        ]
                    }
                ]
            )?,
            "\n| 💻 **Technology** | 🚀 **Projects** |\n| - | - |\n| [![Golang](https://img.shields.io/static/v1?label=&message=Golang&color=7FD6EA&logo=go&logoColor=201020)](https://golang.org/) | ![fgh](https://img.shields.io/static/v1?label=&message=fgh&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) |\n| [![Python](https://img.shields.io/static/v1?label=&message=Python&color=3C78A9&logo=python&logoColor=FFFFFF)](https://www.python.org/) | ![profile_stack](https://img.shields.io/static/v1?label=&message=profile_stack&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) ![test](https://img.shields.io/static/v1?label=&message=test%20(WIP)&color=000605&logo=github&logoColor=FFFFFF&labelColor=000605) |"
        );

        Ok(())
    }
}
