use tracing::info;

mod conf;
mod readme;

fn main() {
    tracing_subscriber::fmt::init();

    let env_var_conf = conf::env_vars().expect("Failed to get env var config");
    let file_conf = conf::config_file(&env_var_conf)
        .expect("Failed to get configuration from file (CHECK FOR NEW UPDATE)");
    info!("Got configuration inputs");

    let table = readme::gen_table(&env_var_conf, &file_conf).expect("Failed to generate table");
    info!("Generated table");

    println!("{:?}", table);
}
