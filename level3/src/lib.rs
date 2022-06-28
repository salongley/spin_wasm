use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{internal_server_error, Request, Response},
    http_component, redis,
};

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