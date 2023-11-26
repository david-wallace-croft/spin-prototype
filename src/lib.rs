use anyhow::Result;
use spin_sdk::{
  http::{IntoResponse, Request},
  http_component,
  key_value::Store,
};
#[http_component]
fn handle_request(_req: Request) -> Result<impl IntoResponse> {
  let store = Store::open_default()?;
  store.set("mykey", b"myvalue")?;
  let value = store.get("mykey")?;
  let response = value.unwrap_or_else(|| "not found".into());
  Ok(http::Response::builder().status(200).body(response)?)
}
