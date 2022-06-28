use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const SERVICE_URL_ENV: &str = "SERVICE_URL";

/// A simple Spin HTTP component.
#[http_component]
fn send_outbound(_: Request) -> Result<Response> {
  let service_url = std::env::var(SERVICE_URL_ENV)?;

  let mut res = spin_sdk::outbound_http::send_request(
    http::Request::builder()
    .method("GET")
    .uri(service_url)
    .body(None)?,
  )?;
   
    res.headers_mut()
        .insert("spin-component","rust-outbound-http".try_into()?);

    Ok(res)
}
