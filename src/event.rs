use rhai::{Engine, Scope, AST};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub struct Event {
    engine: Engine,
    ast: AST,
}

impl Event {
    pub fn connect(&self, scope: &mut Scope) -> bool {
        self.engine
            .call_fn(scope, &self.ast, "connect", ())
            .unwrap_or(true)
    }
}

impl FromStr for Event {
    type Err = rhai::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut engine = Engine::new();

        engine
            .register_type::<SocketAddr>()
            .register_fn("ip", |a: &mut SocketAddr| a.ip())
            .register_fn("port", |a: &mut SocketAddr| a.port())
            .register_fn("to_string", |a: &mut SocketAddr| a.to_string())
            .register_fn("to_debug", |a: &mut SocketAddr| format!("{:?}", a));

        engine
            .register_type::<IpAddr>()
            .register_fn("to_string", |a: &mut IpAddr| a.to_string())
            .register_fn("to_debug", |a: &mut IpAddr| format!("{:?}", a));

        #[allow(deprecated)]
        engine.on_def_var(|_, info, _| Ok(info.name != "ctx"));

        let ast = engine.compile(s)?;

        Ok(Self { engine, ast })
    }
}
