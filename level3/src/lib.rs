use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{internal_server_error, Request, Response},
    http_component, redis,
};
use std::{str, collections::HashMap};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Parameters {
    args: Vec<String>,
    vars: HashMap<String, String>,
    body: Option<Value>,
}

fn get_parameters(request: Request) -> Parameters {
    let mut body = None;

    if str::from_utf8(request.headers().get("content-length").unwrap().as_ref()).unwrap().parse::<usize>().unwrap() != 0 {
        if let Ok(body_json) = serde_json::from_str(str::from_utf8(request.body().as_ref().unwrap().as_ref()).unwrap()) {
            body = Some(body_json);
        }
    }

    Parameters { 
        args: request.uri().query().unwrap_or("None").split('&').map(String::from).collect(),
        vars: request.headers().iter().map(|h| (String::from(h.0.as_str()), String::from(str::from_utf8(h.1.as_ref()).unwrap()))).collect(), 
        body 
    }
}
// The environment variable set in `spin.toml` that points to the
// address of the Redis server that the component will publish
// a message to.
const REDIS_ADDRESS_ENV: &str = "REDIS_ADDRESS";

// The environment variable set in `spin.toml` that specifies
// the Redis channel that the component will publish to.
const REDIS_CHANNEL_ENV: &str = "REDIS_CHANNEL";

/// This HTTP component demonstrates fetching a value from Redis
/// by key, setting a key with a value, and publishing a message
/// to a Redis channel. The component is triggered by an HTTP
/// request served on the route configured in the `spin.toml`.
#[http_component]
fn publish(req: Request) -> Result<Response> {
    let address = std::env::var(REDIS_ADDRESS_ENV)?;
    let channel = std::env::var(REDIS_CHANNEL_ENV)?;

    let data = get_parameters(req);
    
    // Get the message to publish from the Redis key "mykey"
    let payload = redis::get(&address, &"mykey").map_err(|_| anyhow!("Error querying Redis"))?;

    // Set the Redis key "spin-example" to value "Eureka!"
    redis::set(&address, &"spin-example", &b"Eureka Again!"[..])
        .map_err(|_| anyhow!("Error executing Redis command"))?;

    // Publish to Redis
    match redis::publish(&address, &channel, &payload) {
        Ok(()) => Ok(http::Response::builder().status(200).body()?),
        Err(_e) => internal_server_error(),
    }
}