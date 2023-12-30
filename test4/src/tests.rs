use super::*;

#[test]
fn test_make_include_prompt() {
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
      Fuzzy, Squeaky, and Whiskers. ",
  );
}
