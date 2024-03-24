use std::future::Future;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let name1 = "Try".to_string();
    let name2 = "Lindsey".to_string();
    
    say_hello(&name1).await;
    say_hello(&name2).await;
    
    block_on(say_hello2(&name1));
    block_on(say_hello2(&name2));
}

async fn say_hello(name: &str) -> usize {
    println!("Hello, {name}");
    42
}

#[allow(clippy::all)]
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output=usize> + 'fut {
    async move { 
        println!("Hello, {name}");
        42
    }
}
