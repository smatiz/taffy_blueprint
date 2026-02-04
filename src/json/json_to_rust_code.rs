#[cfg(test)]
mod tests {
    use taffy::prelude::*;

    #[test]
    fn test_() {
        let json_text = r#"
{
  "id": "x",
  "style": {
    "size": "400.0 500.0",
    "display": "Flex",
    "justify_content": "Start",
    "align_items": "Start"
  },
  "children": [
    {
      "id": "y",
      "style": {
        "size": "300.0 200.0"
      },
      "children": [],
      "debug_label": "Center #FF0000 *"
    }
  ],
  "debug_label": "E #FF0000 two"
}

"#;
    }
}
