mod config;
mod context;
mod event;

use crate::config::Config;
use crate::context::Context;
use crate::event::Event;
use color_eyre::eyre::eyre;
use rhai::{Dynamic, Map, Scope};
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::{fs, task};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config_dir = dirs::config_dir()
        .map(|p| p.join("rhea"))
        .ok_or_else(|| eyre!("Unable to locate config directory path"))?;

    let config = Arc::new(Config::load(config_dir.join("rhea.toml")).await?);

    let script = fs::read_to_string(config_dir.join("rhea.rhai"))
        .await
        .unwrap_or_default();

    let event = Arc::new(Event::from_str(&script)?);

    let ctx = Context::new(config, event);

    let listener = TcpListener::bind(ctx.config.server.addr).await?;

    loop {
        let (stream, addr) = listener.accept().await?;

        let ctx = ctx.clone();

        tokio::spawn(async move {
            let mut stream = stream;

            let mut map = Map::new();
            map.insert("addr".into(), Dynamic::from(addr));

            if let Err(_err) = handle(&mut stream, ctx, map).await {
                todo!()
            }

            stream.shutdown().await
        });
    }
}

async fn handle(stream: &mut TcpStream, ctx: Context, map: Map) -> color_eyre::Result<()> {
    let mut scope = Scope::new();
    scope.push("ctx", map);

    let event = ctx.event.clone();
    let (allow, mut scope) = task::spawn_blocking(move || {
        let val = event.connect(&mut scope);

        (val, scope)
    })
    .await?;

    if !allow.unwrap_or(true) {
        return Ok(());
    }

    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;

    // TODO: Check if the socks version is correct

    let methods = buf[1] as usize;
    let mut buf = vec![0u8; methods];
    stream.read_exact(&mut buf).await?;

    Ok(())
}
