use clap::Parser;
use spacepls::blueprint::Upstream;
use spacepls::config::reader::ConfigReader;
use crate::command::{Cli, Command};

pub async fn run() -> anyhow::Result<()> {
    logger_init();
    let cli = Cli::parse();
    let runtime = crate::runtime::init(&Upstream::default());
    let config_reader = ConfigReader::init(runtime.clone());
    match cli.command {
        Command::Start { config_path } => {
            let config = config_reader.read(&config_path).await?;
            let server = crate::server::Server::new(config);
            server.fork_start().await?;
        }
        Command::Init { .. } => {}
    }
    Ok(())
}

fn logger_init() {
    // set the log level
    const LONG_ENV_FILTER_VAR_NAME: &str = "SPACEPLS_LOG_LEVEL";
    const SHORT_ENV_FILTER_VAR_NAME: &str = "SP_LOG_LEVEL";

    // Select which env variable to use for the log level filter. This is because filter_or doesn't allow picking between multiple env_var for the filter value
    let filter_env_name = std::env::var(LONG_ENV_FILTER_VAR_NAME)
        .map(|_| LONG_ENV_FILTER_VAR_NAME)
        .unwrap_or_else(|_| SHORT_ENV_FILTER_VAR_NAME);

    // use the log level from the env if there is one, otherwise use the default.
    let env = env_logger::Env::new().filter_or(filter_env_name, "info");

    env_logger::Builder::from_env(env).init();
}
