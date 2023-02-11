use async_trait::async_trait;

struct Data {
    name: String,
    id: i32,
}

impl Data {
    async fn owned_fn(&self) -> u8 {
        println!("owned filed {}, {}", self.name, self.id);
        0
    }
}

async fn foo(data: Data) -> u8 {
    data.owned_fn().await
}

async fn foo_ref<'a>(data: &'a Data) -> u8 {
    data.owned_fn().await
}

struct S<'a, F>
    where
        F: std::future::Future + Send + 'a,
{
    name: String,
    foo_ref: fn(&'a Data) -> F,
    // foo: fn(Data) -> F
}

impl<'a, F> S<'a, F>
    where
        F: std::future::Future + Send,
{
    fn normal_func(&self) {
        println!("name is {}", self.name);
    }
}

async fn example() {
    let name = String::from("name");
    let s = S { name, foo_ref };
    let data = Data{name:"data".to_string(),id:1};
    (s.foo_ref)(&data).await;
    s.normal_func();
}


#[cfg(test)]
mod test {
    use crate::async_filed::{Data, example, foo_ref};

    #[tokio::test]
    async fn test() {
        example().await;
    }
}