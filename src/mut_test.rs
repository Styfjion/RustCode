struct Inner {
    id: i32,
    name: String,
}

impl Inner {
    fn set_id(&mut self, id: &i32) {
        self.id = *id
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn set_name_str(&mut self, name: &str) {
        self.name = name.to_string()
    }
}

struct Data {
    name: String,
    id: i32,
    inner: Inner,
    stub: Option<Inner>,
}

impl Data {
    fn test_fun(&mut self) {
        if let Some(val) = self.check() {
            //self.inner.set_id(&val.id); // val引用持续存在，val为self函数的返回值引用，生命周期和self相同，因此此处和self.inner同时存在，冲突
            //self.inner.set_name_str(&val.name); // 同上
            self.inner.set_name(val.name.clone()); // 基于声明周期NLL规则，val使用完毕，编译器认为val的生命周期已经结束，此处编译通过
            //self.inner.set_name(val.name.clone()); // 再次使用val，val的生命周期延续，因此不注释此行的话，上面一行报错
        }
    }

    fn check(&self) -> Option<&Inner> {
        if let Some(val) = &self.stub {
            Some(val)
        } else {
            None
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn change(id: &mut i32, name: &str) {
    *id = 4;
    println!("name is {}", name)
}

fn set_name(data: &mut Data, name: String) {
    data.name = name;
}

#[cfg(test)]
mod test {
    use crate::mut_test::{change, Data, Inner, set_name};

    #[test]
    fn test_fun() {
        let inner = Inner {
            id: 4,
            name: "new_opt".to_string(),
        };
        let mut data = Data {
            name: "test".to_string(),
            id: 1,
            inner: Inner {
                id: 2,
                name: "inner".to_string(),
            },
            stub: Some(inner),
        };
        data.test_fun();
        println!("{}", data.inner.name);

        change(&mut data.id, &data.name);

        data.set_name(data.inner.name.clone());
        println!("{}", data.name)
        //set_name(&mut data, data.name.clone());
    }
}
