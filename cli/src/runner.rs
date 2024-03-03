pub fn run() -> anyhow::Result<()> {
    logger_init();

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
