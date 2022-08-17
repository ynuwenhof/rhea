use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use toml::de;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    server: Server,
}

impl FromStr for Config {
    type Err = de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Server {
    addr: SocketAddr,
}
