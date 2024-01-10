fn test_string() {
    // get_string为临时变量
    let prime = get_string();
    let after: Vec<&str> = prime.split(",").collect();
    // let after: Vec<&str> = get_string().split(",").collect();
    after.iter().for_each(|line|{print_string(&line.to_uppercase());});
}

fn get_string() -> String {
        "Hello, world".to_string()
}

fn print_string(input: &str) {
    println!("current str is {}", input);
}

#[cfg(test)]
mod test {
    use crate::string_test::test_string;

    #[test]
    fn test() {
        test_string()
    }
}