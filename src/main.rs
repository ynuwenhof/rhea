mod config;
mod context;
mod event;

use crate::config::Config;
use crate::event::Event;
use color_eyre::eyre::eyre;
use rhai::{Dynamic, Map, Scope};
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::{fs, task};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config_dir = dirs::config_dir()
        .map(|p| p.join("rhea"))
        .ok_or_else(|| eyre!("Unable to locate config directory path"))?;

    let config = Config::load(config_dir.join("rhea.toml")).await?;

    let script = fs::read_to_string(config_dir.join("rhea.rhai"))
        .await
        .unwrap_or_default();

    let event = Arc::new(Event::from_str(&script)?);

    let listener = TcpListener::bind(config.server.addr).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let event = event.clone();

        tokio::spawn(async move {
            let mut stream = stream;

            let mut ctx = Map::new();
            ctx.insert("addr".into(), Dynamic::from(addr));

            let mut scope = Scope::new();
            scope.push("ctx", ctx);

            let evnt = event.clone();
            let (allow, mut scope) = task::spawn_blocking(move || {
                let val = evnt.connect(&mut scope);

                (val, scope)
            })
            .await?;

            if allow.unwrap_or(true) {
                todo!()
            }

            stream.shutdown().await
        });
    }
}
