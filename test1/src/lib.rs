use anyhow::Result;
use spin_sdk::http::{HeaderValue, IntoResponse, Request};
use spin_sdk::http_component;

#[http_component]
fn handle_request(request: Request) -> Result<impl IntoResponse> {
  let header_value_option: Option<&HeaderValue> =
    request.header("spin-full-url");
  let body: String = match header_value_option {
    Some(header_value) => {
      let str_option: Option<&str> = header_value.as_str();
      match str_option {
        Some(str_value) => format!("Header spin-full-url: {str_value}"),
        None => "Header spin-full-url is not a string".to_owned(),
      }
    },
    None => "Header spin-full-url is not set".to_owned(),
  };
  Ok(
    http::Response::builder()
      .status(200)
      .header("Content-Type", "text/plain")
      .body(body)?,
  )
}
