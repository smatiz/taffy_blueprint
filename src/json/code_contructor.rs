use serde_json::Value;

fn to_rust_code(v: &Value) -> String {
    match v {
        Value::Null => "None".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\".to_string()", s),
        Value::Array(arr) => {
            let items = arr.iter().map(to_rust_code).collect::<Vec<_>>().join(", ");
            format!("vec![{}]", items)
        }
        Value::Object(obj) => {
            let fields = obj
                .iter()
                .map(|(k, v)| format!("{}: {}", k, to_rust_code(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("MyType {{ {} }}", fields)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_() {
        let s = std::fs::read_to_string("example.json").unwrap();
        let v: Value = serde_json::from_str(&s).unwrap();
        println!("rust: {}", v);
    }
}
