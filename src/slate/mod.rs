#![allow(dead_code)]

use serde_json::{Map, Value};

pub fn plain_serialize(value: &Value) -> String {
    if is_iterable(&value) {
        if let Some(Value::Array(nodes)) = value.get("nodes") {
            nodes
                .iter()
                .filter(|node| !is_type_code(&node))
                .map(|node| plain_serialize(&node))
                .fold(String::new(), fold_text_nodes)
                .trim()
                .to_string()
        } else {
            String::from("")
        }
    } else if is_text_node(&value) {
        if let Some(Value::Array(leaves)) = value.get("leaves") {
            leaves
                .iter()
                .map(|node| plain_serialize(&node))
                .fold(String::new(), fold_text_nodes)
                .trim()
                .to_string()
        } else {
            String::from("")
        }
    } else if is_root(&value) {
        plain_serialize(value.get("document").unwrap())
    } else if has_text_field(&value) {
        if let Some(Value::String(text)) = value.get("text") {
            text.trim().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn fold_text_nodes(acc: String, text: String) -> String {
    if text.trim().len() > 0 {
        acc + " " + text.trim()
    } else {
        acc
    }
}

fn is_text_node(value: &Value) -> bool {
    if let Value::Object(map) = value {
        check_value(&map, "object", is_text) && check_value(&map, "leaves", |node| node.is_array())
    } else {
        false
    }
}

fn has_text_field(value: &Value) -> bool {
    if let Value::Object(map) = value {
        map.get("text").is_some()
    } else {
        false
    }
}

fn is_root(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            check_value(&map, "object", is_value)
                && check_value(&map, "document", |node| node.is_object())
        }
        _ => false,
    }
}

fn is_iterable(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            check_value(&map, "object", is_document)
                || (check_value(&map, "object", is_block)
                    && check_value(&map, "nodes", |node| node.is_array()))
        }
        _ => false,
    }
}

#[inline]
fn is_value(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "value",
        _ => false,
    }
}

#[inline]
fn is_document(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "document",
        _ => false,
    }
}

#[inline]
fn is_block(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "block",
        _ => false,
    }
}

fn is_type_code(value: &Value) -> bool {
    match value {
        Value::Object(map) => check_value(&map, "type", is_code),
        _ => false,
    }
}

fn is_code(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "code",
        _ => false,
    }
}

fn is_text(value: &Value) -> bool {
    if let Value::String(s) = value {
        s == "text"
    } else {
        false
    }
}

fn check_value<V, S>(map: &Map<String, Value>, name: S, validator: V) -> bool
where
    V: Fn(&Value) -> bool,
    S: ToString,
{
    if let Some(value) = map.get(&name.to_string()) {
        validator(value)
    } else {
        false
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn slate_document_serialized_to_plain_string_without_html() {
        use super::plain_serialize;
        use serde_json::Value;

        let json = include_str!("example_slate_document.json");
        let value: Value = serde_json::from_str(&json).unwrap();

        let result = plain_serialize(&value);
        let expected = r#"bold italic underline blockquote list 32"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn slate_document_serialized_to_plain_string_big_example() {
        use super::plain_serialize;
        use serde_json::Value;

        let json = include_str!("big_example.json");
        let value: Value = serde_json::from_str(&json).unwrap();

        let result = plain_serialize(&value);
        let expected = r#"one two three one-1 two-1 bold text-test-ltialic .    ret4eg4534regt34rw 43 three-1 цитатацитатацитатацитатацитатацитатацитатацитатацитата цитатацитатацитатацитатацитатацитатацитатацитата цитатацитатацитата"#;

        assert_eq!(result, expected);
    }
}
