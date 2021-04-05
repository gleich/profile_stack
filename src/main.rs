use tracing::info;

mod conf;

fn main() {
    tracing_subscriber::fmt::init();

    let env_var_conf = conf::env_vars().expect("Failed to get env var config");
    let file_conf = conf::config_file(&env_var_conf);
    info!("Got configuration inputs");

    println!("{:?}", file_conf);
}
