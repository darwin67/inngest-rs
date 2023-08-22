use axum::{
    routing::{get, put},
    Router,
};
use inngest::{
    event::Event,
    function::{create_function, FunctionOps, Input, ServableFunction, Trigger},
    router::{axum as inngest_axum, Handler},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut inngest_handler = Handler::new();
    inngest_handler.register_fn(dummy_fn());
    inngest_handler.register_fn(hello_fn());

    let inngest_state = Arc::new(inngest_handler);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/inngest", put(inngest_axum::register))
        .with_state(inngest_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct DummyEvent {
    data: u8,
}

#[typetag::serde]
impl Event for DummyEvent {
    fn id(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        "test/event".to_string()
    }

    fn data(&self) -> &dyn std::any::Any {
        &self.data
    }

    fn user(&self) -> Option<&dyn std::any::Any> {
        None
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }

    fn version(&self) -> Option<String> {
        None
    }
}

fn dummy_fn() -> Box<dyn ServableFunction + Sync + Send> {
    create_function(
        FunctionOps {
            name: "Dummy func".to_string(),
            ..Default::default()
        },
        Trigger::EventTrigger {
            event: "test/event".to_string(),
            expression: None,
        },
        Box::new(|input: Input<&dyn Event>| {
            println!("In dummy function");

            let evt = input.event;
            println!("Event: {}", evt.name());
            println!("Data: {:?}", evt.data());

            Ok(Box::new("test result".to_string()))
        }),
    )
}

fn hello_fn() -> Box<dyn ServableFunction + Sync + Send> {
    create_function(
        FunctionOps {
            name: "Hello func".to_string(),
            ..Default::default()
        },
        Trigger::EventTrigger {
            event: "test/hello".to_string(),
            expression: None,
        },
        Box::new(|input: Input<&dyn Event>| {
            println!("In hello function");

            let evt = input.event;
            println!("Event: {}", evt.name());
            println!("Data: {:?}", evt.data());

            Ok(Box::new("test hello".to_string()))
        }),
    )
}
