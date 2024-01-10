use http::StatusCode;
use spin_sdk::{
  http::{IntoResponse, Params, Request, Response, Router},
  http_component,
  key_value::Store,
};

const KEY: &str = "key";
const KEY_NOT_FOUND: &str = "key not found";
const PATH: &str = "/test2/:key";

#[http_component]
fn handle_request(request: Request) -> Response {
  let mut router = Router::new();
  router.delete(PATH, handle_delete);
  router.get(PATH, handle_get);
  router.head(PATH, handle_head);
  router.post(PATH, handle_post);
  router.handle(request)
}

fn handle_delete(
  _request: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key: &str = params.get(KEY).expect(KEY_NOT_FOUND);
  let store: Store = Store::open_default()?;
  store.delete(key)?;
  let response = Response::new(StatusCode::OK, None::<Vec<u8>>);
  Ok(response)
}

fn handle_get(
  _request: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key: &str = params.get(KEY).expect(KEY_NOT_FOUND);
  let store: Store = Store::open_default()?;
  let value: Option<Vec<u8>> = store.get(key)?;
  match value {
    Some(value) => Ok(Response::new(StatusCode::OK, value)),
    None => Ok(Response::new(StatusCode::NOT_FOUND, None::<Vec<u8>>)),
  }
}

fn handle_head(
  _request: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key: &str = params.get(KEY).expect(KEY_NOT_FOUND);
  let store: Store = Store::open_default()?;
  let value: Option<Vec<u8>> = store.get(key)?;
  let code: StatusCode = if value.is_some() {
    StatusCode::OK
  } else {
    StatusCode::NOT_FOUND
  };
  let response = Response::new(code, None::<Vec<u8>>);
  Ok(response)
}

fn handle_post(
  request: Request,
  params: Params,
) -> anyhow::Result<impl IntoResponse> {
  let key: &str = params.get(KEY).expect(KEY_NOT_FOUND);
  let store: Store = Store::open_default()?;
  let request_body: &[u8] = request.body();
  store.set(key, request_body)?;
  let response = Response::new(StatusCode::OK, None::<Vec<u8>>);
  Ok(response)
}
