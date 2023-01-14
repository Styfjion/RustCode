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

    #[test]
    fn test_serde() {
        let input_str = vec!["{\"integer\":2}",
                             "{\"string\": \"test\"}",
                             "{\"message\":{\"_type\":\"type_1\", \"body\": \"test_body\"}}",
                             "{\"anonymous\":{\"x\": 1, \"y\": \"test_y\"}}",
                             "{\"tuple\":[1, \"test_2\", 1.2]}"];
        for item in &input_str {
            match serde_json::from_str::<Input>(item) {
                Ok(val) => {
                    match val {
                        Input::Integer(integer) => println!("Integer val is {}", integer),
                        Input::String(str) => println!("String is {}", str),
                        Input::Struct(message) => println!("Struct is {:?}, struct value is type:{} and body:{}", message, message._type, message.body),
                        Input::Tuple(i, j, k) => println!("Tuple is {}, {}, and {}", i, j, k),
                        Input::Anonymous { x, y } => println!("Anonymous is {} and {}", x, y)
                    }
                }
                Err(error) => println!("error is {:?}", error)
            }
        }
    }
}

