use color_eyre::eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use tokio::fs;
use toml::de;

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> color_eyre::Result<Self> {
        fs::read_to_string(path)
            .await
            .map(|s| s.parse())
            .unwrap_or_else(|_| Ok(Self::default()))
            .wrap_err("Unable to parse config")
    }
}

impl FromStr for Config {
    type Err = de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    #[serde(default = "default_socket_addr")]
    pub addr: SocketAddr,
    #[serde(default)]
    pub socks: Socks,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            addr: default_socket_addr(),
            socks: Default::default(),
        }
    }
}

fn default_socket_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Socks {
    #[serde(default)]
    pub auth: bool,
    #[serde(default)]
    pub no_auth: bool,
}

impl Default for Socks {
    fn default() -> Self {
        Self {
            auth: false,
            no_auth: true,
        }
    }
}
