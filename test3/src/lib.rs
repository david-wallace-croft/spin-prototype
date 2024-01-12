use anyhow::Result;
use http::{Method, Request, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
use spin_sdk::http_component;

#[derive(Deserialize)]
struct Input {
  name: String,
}

#[derive(Serialize)]
struct Output {
  message: String,
}

impl IntoBody for Output {
  fn into_body(self) -> Vec<u8> {
    let result: Result<String, serde_json::Error> =
      serde_json::to_string(&self);
    let json: String = result.unwrap();
    json.into_body()
  }
}

#[http_component]
fn handle_request(request: Request<Json<Input>>) -> Result<impl IntoResponse> {
  let method: &Method = request.method();
  let (status, body): (StatusCode, Option<Output>) = match *method {
    Method::POST => {
      let json_input: &Json<Input> = request.body();
      let output = Output {
        message: format!("Hello, {}!", json_input.name),
      };
      (StatusCode::OK, Some(output))
    },
    _ => (StatusCode::METHOD_NOT_ALLOWED, None),
  };
  let response = Response::builder()
    .body(body)
    .header("Content-Type", "application/json")
    .status(status)
    .build();
  Ok(response)
}
