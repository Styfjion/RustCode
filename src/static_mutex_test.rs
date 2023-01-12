use std::sync::Mutex;

static GLOBAL_MUTEX: Mutex<Vec<u8>> = Mutex::new(vec![]);

async fn do_a_call() {
    GLOBAL_MUTEX.lock().unwrap().push(1);
}

pub async fn test_static_mutex() {
    do_a_call().await;
    let task_1 = tokio::spawn(do_a_call());
    let task_2 = tokio::spawn(do_a_call());
    task_1.await.unwrap();
    task_2.await.unwrap();
    println!("called {}", GLOBAL_MUTEX.lock().unwrap().len());
}

#[cfg(test)]
mod tests {
    use crate::static_mutex_test::test_static_mutex;

    #[tokio::test]
    async fn test() {
        test_static_mutex().await;
    }
}