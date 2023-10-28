use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str;

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN]
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..bytes.len()].copy_from_slice(bytes);
        Self {
            len: bytes.len() as u8,
            data
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String)
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MyString::Inline(mini_str) => mini_str.deref(),
            MyString::Standard(standard_str) => standard_str
        }
    }
}

impl<T> From<T> for MyString where T: AsRef<str> {
    fn from(value: T) -> Self {
        match value.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(value.as_ref().to_owned()),
            _ => Self::Inline(MiniString::new(value))
        }
    }
}

impl MyString {
    pub fn push_str(&mut self, s: &str) {
        match self {
            Self::Inline(mini_string) => {
                if mini_string.len() + s.len() <= MINI_STRING_MAX_LEN {
                    mini_string.data[mini_string.len as usize..].copy_from_slice(s.as_bytes());
                    mini_string.len += s.len() as u8;
                } else {
                    let mut new_string = mini_string.deref().to_string();
                    new_string.push_str(s);
                    *self = Self::Standard(new_string);
                }
            },
            Self::Standard(standard) => {
                standard.push_str(s);
            }
        }
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with('这'));
}