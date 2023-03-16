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

    #[test]
    fn test_two() {
        let text = "2.3.1";

        let re = Regex::new("(\\d)+\\.(\\d)+\\.(\\d)+?").unwrap();
        let caps = re.captures(text).unwrap();
        println!("All is {}", caps[0].to_string());
        println!("One is {}", caps[1].to_string());
        println!("Two is {}", caps[2].to_string());
        println!("Three is {}", caps[3].to_string());
    }

    #[test]
    fn test_named() {
        let text = "2.3.1";

        let re = Regex::new("(?P<prime>\\d)+\\.(?P<main>\\d)+\\.(?P<patch>\\d)+?").unwrap();
        let caps = re.captures(text).unwrap();
        println!("All is {}", caps[0].to_string());
        println!("prime is {}", caps.name("prime").unwrap().as_str());
        println!("main is {}", caps.name("main").unwrap().as_str());
        println!("patch is {}", caps.name("patch").unwrap().as_str());
    }
}
