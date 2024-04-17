mod config;
mod bot;

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version = VERSION, about = "CatALog")]
struct Opts {
    #[clap(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    pretty_env_logger::init();

    let cfg = config::config::read(opts.config).unwrap();

    bot::core::start_bot(cfg.core_config).await;
}