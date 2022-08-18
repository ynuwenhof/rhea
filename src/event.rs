use rhai::{Engine, Scope, AST};
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

        #[allow(deprecated)]
        engine.on_def_var(|_, info, _| Ok(info.name != "ctx"));

        let ast = engine.compile(s)?;

        Ok(Self { engine, ast })
    }
}
