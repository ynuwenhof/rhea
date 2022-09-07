use crate::{Config, Event};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub event: Arc<Event>,
}

impl Context {
    pub fn new(config: Arc<Config>, event: Arc<Event>) -> Self {
        Self { config, event }
    }
}
