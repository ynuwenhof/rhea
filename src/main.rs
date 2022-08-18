use crate::config::Config;
use color_eyre::eyre::eyre;

mod config;
mod event;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config_dir = dirs::config_dir()
        .map(|p| p.join("rhea"))
        .ok_or_else(|| eyre!("Unable to locate config directory path"))?;

    let _config = Config::load(config_dir.join("rhea.toml")).await?;

    Ok(())
}
