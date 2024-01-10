use async_trait::async_trait;

#[async_trait]
trait Step {
    async fn run(&self) -> u8;
}

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

async fn foo_ref(data: &Data) -> u8 {
    data.owned_fn().await
}

struct S<'a, F>
    where
        F: std::future::Future + Send + 'a,
{
    name: String,
    foo_ref: fn(&'a Data) -> F,
}

impl<'a, F> S<'a, F>
    where
        F: std::future::Future + Send + 'a,
{
    async fn normal_func(&self, data: &'a Data) {
        (self.foo_ref)(data).await;
    }
}

#[async_trait]
impl<'a, F> Step for S<'a, F>
    where
        F: std::future::Future + Send + 'a, {
    async fn run(&self) -> u8 {
        println!("run");
        0
    }
}

async fn example() {
    let name = String::from("name");
    let s = S { name, foo_ref };
    let data = Data{name:"data".to_string(),id:1};
    s.normal_func(&data).await;
    s.run().await;
}


#[cfg(test)]
mod test {
    use crate::async_filed::{example, foo_ref};

    #[tokio::test]
    async fn test() {
        example().await;
    }
}