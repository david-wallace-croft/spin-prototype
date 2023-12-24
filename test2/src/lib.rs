use http::StatusCode;
use spin_sdk::{
  http::{IntoResponse, Params, Request, Response, Router},
  http_component,
  key_value::Store,
};

#[http_component]
fn handle_request(req: Request) -> Response {
  let mut router = Router::new();
  router.delete("/test2/:key", handle_delete);
  router.get("/test2/:key", handle_get);
  router.head("/test2/:key", handle_head);
  router.post("/test2/:key", handle_post);
  router.handle(req)
}

fn handle_delete(
  _req: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key = params.get("key").expect("key not found");
  println!("Delete key {key}");
  let store = Store::open_default()?;
  store.delete(key)?;
  Ok(Response::new(StatusCode::OK, None::<Vec<u8>>))
}

fn handle_get(
  _req: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key = params.get("key").expect("key not found");
  println!("Get key {key}");
  let store = Store::open_default()?;
  let value: Option<Vec<u8>> = store.get(key)?;
  match value {
    Some(value) => Ok(Response::new(StatusCode::OK, value)),
    None => Ok(Response::new(StatusCode::NOT_FOUND, None::<Vec<u8>>)),
  }
}

fn handle_head(
  _req: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key = params.get("key").expect("key not found");
  println!("Head key {key}");
  let store = Store::open_default()?;
  let value: Option<Vec<u8>> = store.get(key)?;
  let code = if value.is_some() {
    StatusCode::OK
  } else {
    StatusCode::NOT_FOUND
  };
  Ok(Response::new(code, None::<Vec<u8>>))
}

fn handle_post(
  req: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key = params.get("key").expect("key not found");
  println!("Post key {key}");
  let store = Store::open_default()?;
  store.set(key, req.body())?;
  Ok(Response::new(StatusCode::OK, None::<Vec<u8>>))
}
