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

        // let mut style = Style::<String>::default();
        // style.align_content = None;
        // let s = serde_json5::to_string(&style.align_content);
        // println!("s {:?}", s);
        // style.align_content = Some(taffy::AlignContent::FlexEnd);
        // let s = serde_json5::to_string(&style.align_content);
        // println!("s {:?}", s);

        // style.size = taffy::Size {
        //     width: length(123.4),
        //     height: auto(),
        // };
        // if let Some(s) = serde_json5::to_string(&style.size.width).ok() {
        //     println!("s {:?}", s);
        // }

        // let compact_length = style.size.width.into_raw();
        // compact_length.uses_percentage()
        // match  {}
    }
}
