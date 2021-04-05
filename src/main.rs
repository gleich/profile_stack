mod conf;

fn main() {
	tracing_subscriber::fmt::init();

	let env_vars = conf::Env::get().expect("Failed to get env var config");
	println!("{:?}", env_vars);
}
