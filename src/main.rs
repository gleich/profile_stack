use std::fs;
use std::io::{Read, Write};
use std::process::Command;

use fs::File;
use tracing::{info, warn};

mod conf;
mod readme;

fn main() {
    tracing_subscriber::fmt::init();

    // Getting configuration
    let env_var_conf = conf::env_vars().expect("Failed to get env var config");
    let file_conf = conf::config_file(&env_var_conf)
        .expect("Failed to get configuration from file (CHECK FOR NEW UPDATE)");
    info!("Got configuration inputs");

    // Generating table
    let table = readme::gen_table(&env_var_conf, &file_conf).expect("Failed to generate table");
    info!("Generated table");

    // Inserting table into README
    let mut readme_file =
        File::open(&readme::FILE_NAME).expect("Failed to create README.md file struct");
    let mut readme_content = String::new();
    readme_file
        .read_to_string(&mut readme_content)
        .expect(&format!("Failed to read from {}", readme::FILE_NAME));

    let patched_content = readme::insert_table(&readme_content, &table)
        .expect("Failed to insert table to README data");

    // Writing the changes to the README
    if readme_content != patched_content {
        // Writing changes
        readme_file
            .write_all(patched_content.as_bytes())
            .expect(&format!("Failed to write changes to {}", readme::FILE_NAME));
        info!("Wrote changes to {}", readme::FILE_NAME);

        // Committing changes
        let git_program = "git";
        Command::new(git_program)
            .arg("config")
            .arg("--global")
            .arg("user.name")
            .arg("\"action&github.com\"")
            .output()
            .expect("Failed to set commit email");
        Command::new(git_program)
            .arg("config")
            .arg("--global")
            .arg("user.name")
            .arg("\"Publishing Bot\"")
            .output()
            .expect("Failed to set commit email");
        Command::new(git_program)
            .arg("add")
            .arg(readme::FILE_NAME)
            .output()
            .expect("Failed to stage changes");
        Command::new(git_program)
            .arg("commit")
            .arg("-m")
            .arg("\"Update profile stack\"")
            .output()
            .expect("Failed to commit staged changes");
        Command::new(git_program)
            .arg("push")
            .output()
            .expect("Failed to push committed changes");
    } else {
        warn!("No changes to README.md")
    }
}
