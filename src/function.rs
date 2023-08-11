use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Deserialize, Serialize)]
pub trait ServableFunction {
    fn slug() -> String;
    fn name() -> String;
    fn trigger() -> Box<dyn Trigger>;
}

#[derive(Debug, Clone)]
pub struct FunctionOps {
    id: String,
    name: String,
    retries: u8,
}

pub struct Function {
    opts: FunctionOps,
    trigger: Box<dyn Trigger>,
    // TODO: the actual function
}

pub trait Trigger {
    fn trigger(&self) -> String;
    fn expression(&self) -> Option<String>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventTrigger {
    event: String,
    expression: Option<String>,
}

impl Trigger for EventTrigger {
    fn trigger(&self) -> String {
        self.event.clone()
    }

    fn expression(&self) -> Option<String> {
        self.expression.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CronTrigger {
    cron: String,
}

impl Trigger for CronTrigger {
    fn trigger(&self) -> String {
        self.cron.clone()
    }

    fn expression(&self) -> Option<String> {
        None
    }
}