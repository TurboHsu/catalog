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

    let conf = config::config::load_config(opts.config);

    bot::core::start_bot(conf.bot_token).await;
}