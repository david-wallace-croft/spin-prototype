use http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Json, Response};
use spin_sdk::llm::InferencingResult;
use spin_sdk::{http_component, llm};

#[cfg(test)]
mod tests;

#[derive(Clone, Deserialize)]
struct Input {
  #[serde(default)]
  characters: Option<Vec<String>>,
  #[serde(default)]
  objects: Option<Vec<String>>,
  #[serde(default)]
  place: Option<String>,
  #[serde(default)]
  theme: Option<String>,
}

#[derive(Serialize)]
struct Output {
  prompt: String,
  result: String,
  story: String,
}

impl IntoBody for Output {
  fn into_body(self) -> Vec<u8> {
    serde_json::to_string(&self).unwrap().into_body()
  }
}

#[http_component]
fn handle_request(
  req: http::Request<Json<Input>>
) -> anyhow::Result<impl IntoResponse> {
  let (status, body): (StatusCode, Option<Output>) = match *req.method() {
    Method::POST => {
      let json_input: &Json<Input> = req.body();
      let input: Input = json_input.0.clone();
      let output = confabulate(input);
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

fn confabulate(input: Input) -> Output {
  let prompt = make_prompt(input);
  let options = llm::InferencingParams {
    max_tokens: 1000,
    repeat_penalty: 1.2,
    repeat_penalty_last_n_token_count: 0,
    temperature: 0.7,
    top_k: 0,
    top_p: 1.0,
  };
  let infer_result: Result<InferencingResult, spin_sdk::llm::Error> =
    llm::infer_with_options(
      llm::InferencingModel::Llama2Chat,
      &prompt,
      options,
    );
  let result = match &infer_result {
    Ok(inferencing_result) => format!("{:?}", inferencing_result),
    Err(error) => format!("Error: {:?}", error),
  };
  let story = match infer_result {
    Ok(inferencing_result) => inferencing_result.text,
    Err(_error) => String::new(),
  }
  .trim()
  .to_owned();
  Output {
    prompt,
    result,
    story,
  }
}

fn make_include_prompt(
  items: &[String],
  plural: &str,
  singular: &str,
) -> String {
  let items_length = items.len();
  if items_length == 0 {
    return String::new();
  }
  if items_length == 1 {
    return format!(
      "The story should include the following {}: {}.",
      singular, &items[0]
    );
  }
  let mut include_prompt: String =
    format!("The story should include the following {}: ", plural);
  for (i, item) in items.iter().enumerate() {
    if i == items_length - 1 {
      include_prompt.push_str("and ");
    }
    include_prompt.push_str(item);
    if i == items_length - 1 {
      include_prompt.push('.');
    } else {
      include_prompt.push_str(", ");
    }
  }
  include_prompt
}

fn make_prompt(input: Input) -> String {
  let mut prompt = "Tell a story. \
    The story should have a happy ending. \
    The story should be between 250 and 500 words long. "
    .to_owned();
  if let Some(theme) = input.theme {
    prompt.push_str(&format!("The story should have a theme of {}. ", theme));
  }
  if let Some(place) = input.place {
    prompt.push_str(&format!(
      "The story should take place in the following location: {}. ",
      place
    ));
  }
  if let Some(characters) = input.characters {
    prompt.push_str(&make_include_prompt(
      &characters,
      "characters",
      "character",
    ));
  }
  if let Some(objects) = input.objects {
    prompt.push_str(&make_include_prompt(&objects, "objects", "object"));
  }
  prompt
}
