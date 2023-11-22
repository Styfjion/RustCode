use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    _type: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Input {
    #[serde(rename = "integer")]
    Integer(i32),
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "message")]
    Struct(Message),
    #[serde(rename = "tuple")]
    Tuple(i32, String, f32),
    #[serde(rename = "anonymous")]
    Anonymous { x: i32, y: String },
}

#[cfg(test)]
mod tests {
    use crate::serde_enum_struct_test::Input;
    use serde_json::{json, Value};

    #[test]
    fn test_serde() {
        let input_str = vec![
            "{\"integer\":2}",
            "{\"string\": \"test\"}",
            "{\"message\":{\"_type\":\"type_1\", \"body\": \"test_body\"}}",
            "{\"anonymous\":{\"x\": 1, \"y\": \"test_y\"}}",
            "{\"tuple\":[1, \"test_2\", 1.2]}",
        ];
        for item in &input_str {
            match serde_json::from_str::<Input>(item) {
                Ok(val) => match val {
                    Input::Integer(integer) => println!("Integer val is {}", integer),
                    Input::String(str) => println!("String is {}", str),
                    Input::Struct(message) => println!(
                        "Struct is {:?}, struct value is type:{} and body:{}",
                        message, message._type, message.body
                    ),
                    Input::Tuple(i, j, k) => println!("Tuple is {}, {}, and {}", i, j, k),
                    Input::Anonymous { x, y } => println!("Anonymous is {} and {}", x, y),
                },
                Err(error) => println!("error is {:?}", error),
            }
        }
    }

    #[test]
    fn test_serde_obj() {
        let value_vec = vec![
            Value::String("hello".to_string()),
            Value::String(r###"[{"key1":"test", "key2":1}]"###.to_string()),
        ];
        let trans_to_value = |prime: Value| {
            let prime_str = prime.as_str().unwrap_or("");
            serde_json::from_str(prime_str).unwrap_or(prime)
        };
        let new_vec: Vec<_> = value_vec.into_iter().map(trans_to_value).collect();
        println!("{new_vec:?}")
    }
}
