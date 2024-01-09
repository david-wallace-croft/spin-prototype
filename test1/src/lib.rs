use spin_sdk::http::{HeaderValue, IntoResponse, Request};
use spin_sdk::http_component;

#[http_component]
fn handle_test1(request: Request) -> anyhow::Result<impl IntoResponse> {
  let header_value_option: Option<&HeaderValue> =
    request.header("spin-full-url");
  println!("Handling request to {:?}", header_value_option);
  Ok(
    http::Response::builder()
      .status(200)
      .header("Content-Type", "text/plain")
      .body("Hello, World!")?,
  )
}
