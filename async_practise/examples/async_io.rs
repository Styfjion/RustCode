use anyhow::{Error, Result};
use serde_yaml::Value;
use tokio::fs::{read_to_string, write};
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<()> {
    // 读取
    let f1 = read_to_string("Cargo.toml");
    let f2 = read_to_string("Cargo.lock");
    let (content1, content2) = try_join!(f1, f2)?;
    
    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;
    
    // 写入
    let f3 = write("Cargo.yml", &yaml1);
    let f4 = write("Cargo_lock.yml", &yaml2);
    
    try_join!(f3, f4)?;
    
    println!("{yaml1}");
    println!("{yaml2}");
    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    serde_yaml::to_string(&value).map_err(Error::new)
}
