struct Data {
    id: i32,
    name: String
}

fn show(id: &i32, name: &String) {
    println!("id is {id}, name is {name}")
}

fn change(id: &i32, name: &mut String) {
    name.push_str("...");
    show(id, name);
}

#[cfg(test)]
mod test {
    use crate::ref_test::{change, Data, show};

    #[test]
    fn test() {
        let mut data = Data{id: 1, name: "name".to_string()};
        show(&data.id, &data.name);
        change(&data.id, &mut data.name);
    }
}
