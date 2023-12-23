use std::collections::HashMap;

use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::{FromBody, IntoBody};
use spin_sdk::http::{send, IntoResponse, Request, Response};
use spin_sdk::http_component;
use url::Url;

#[derive(Deserialize)]
struct Test5Input {
  name: String,
}

#[derive(Serialize)]
struct Test3Input {
  name: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct Test3Output {
  message: String,
}

#[derive(Serialize)]
struct Test5Output {
  #[serde(alias = "test3Output")]
  test3_output: Test3Output,
}

impl IntoBody for Test3Input {
  fn into_body(self) -> Vec<u8> {
    serde_json::to_string(&self).unwrap().into_body()
  }
}

impl FromBody for Test3Output {
  fn from_body(body: Vec<u8>) -> Self {
    serde_json::from_slice(&body).unwrap()
  }
}

impl IntoBody for Test5Output {
  fn into_body(self) -> Vec<u8> {
    serde_json::to_string(&self).unwrap().into_body()
  }
}

#[http_component]
async fn handle_request(request: Request) -> anyhow::Result<impl IntoResponse> {
  let mut name = "World".to_string();
  let uri: &str = request.uri();
  let parsed_url = Url::parse(uri);
  dbg!(&parsed_url);
  if let Ok(parsed_url) = parsed_url {
    let map: HashMap<String, String> =
      parsed_url.query_pairs().into_owned().collect();
    dbg!(&map);
    let name_value = map.get("name");
    if let Some(name_value) = name_value {
      name = name_value.to_string();
    }
  }
  let test3_input = Test3Input {
    name,
  };
  let outbound_req = Request::post("/test3", test3_input);
  let response: http::Response<Test3Output> = send(outbound_req).await?;
  if response.status() != 200 {
    let response = Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .build();
    return Ok(response);
  }
  let response_body: &Test3Output = response.body();
  let test5_output = Test5Output {
    test3_output: response_body.clone(),
  };
  let response = Response::builder()
    .body(test5_output)
    .header("Content-Type", "application/json")
    .status(StatusCode::OK)
    .build();
  Ok(response)
}
