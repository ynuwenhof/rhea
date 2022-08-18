use rhai::{Engine, AST};
use std::str::FromStr;

pub struct Event {
    engine: Engine,
    ast: AST,
}

impl FromStr for Event {
    type Err = rhai::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let engine = Engine::new();
        let ast = engine.compile(s)?;

        Ok(Self { engine, ast })
    }
}
