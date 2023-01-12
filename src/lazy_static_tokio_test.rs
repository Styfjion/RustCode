use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

async fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

pub async fn test_lazy() {
    do_a_call().await;
    let task_1 = tokio::spawn(do_a_call());
    let task_2 = tokio::spawn(do_a_call());
    task_1.await.unwrap();
    task_2.await.unwrap();
    println!("called {}", ARRAY.lock().unwrap().len());
}

#[cfg(test)]
mod tests {
    use crate::lazy_static_tokio_test::test_lazy;

    #[tokio::test]
    async fn test() {
        test_lazy().await;
        print!("test")
    }
}