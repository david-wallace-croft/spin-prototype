use super::*;

#[test]
fn test_make_include_prompt_plural() {
  let items: Vec<String> = vec![
    "Fuzzy", "Squeaky", "Whiskers",
  ]
  .iter()
  .map(|s| s.to_string())
  .collect();
  let prompt = make_include_prompt(&items, "mice", "mouse");
  assert_eq!(
    prompt,
    "The story should include the following mice: \
      Fuzzy, Squeaky, and Whiskers.",
  );
}

#[test]
fn test_make_include_prompt_singular() {
  let items: Vec<String> =
    vec!["Fuzzy"].iter().map(|s| s.to_string()).collect();
  let prompt = make_include_prompt(&items, "mice", "mouse");
  assert_eq!(
    prompt,
    "The story should include the following mouse: Fuzzy.",
  );
}

#[test]
fn test_make_prompt() {
  let test_input = Input {
    characters: Some(vec![
      "Ada".to_string(),
      "Ben".to_string(),
    ]),
    objects: Some(vec![
      "bat".to_string(),
      "cat".to_string(),
      "hat".to_string(),
    ]),
    place: Some("beach".to_string()),
    theme: Some("joy".to_string()),
  };
  let prompt = make_prompt(test_input);
  assert_eq!(
    prompt,
    "Tell a story. \
      The story should have a happy ending. \
      The story should be between 250 and 500 words long. \
      The story should have a theme of joy. \
      The story should take place in the following location: beach. \
      The story should include the following characters: Ada and Ben. \
      The story should include the following objects: bat, cat, and hat.",
  );
}
