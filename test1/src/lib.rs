use anyhow::Result;
use spin_sdk::http::{HeaderValue, IntoResponse, Request};
use spin_sdk::http_component;

#[http_component]
fn handle_request(request: Request) -> Result<impl IntoResponse> {
  let header_value_option: Option<&HeaderValue> =
    request.header("spin-full-url");
  let body: String = match header_value_option {
    Some(header_value) => format!("spin-full-url: {:?}", header_value),
    None => "spin-full-url not provided".to_owned(),
  };
  Ok(
    http::Response::builder()
      .status(200)
      .header("Content-Type", "text/plain")
      .body(body)?,
  )
}
