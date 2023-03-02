#[cfg(test)]
mod test {
    use regex::Regex;

    #[test]
    fn testfn() {
        let text = "One car red car blue car";

        let re = Regex::new(r"(\w+)\s+(car)").unwrap();

        for (index, caps) in re.captures_iter(text).enumerate() {
            println!("Match{index}");
            for (sub_index, sub) in caps.iter().enumerate() {
                if sub_index == 0 {
                    continue
                }
                let result = sub.unwrap().as_str();
                println!("Capture{sub_index} is {result}");
            }
        }
    }
}
