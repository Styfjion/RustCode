use std::{sync::OnceLock, thread};
use std::sync::Mutex;

pub fn test_once_call() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let global = instance();
        let mut inner = global.lock().unwrap();
        inner.push(2);
    });

    // 主线程调用
    let mut inner = instance().lock().unwrap();
    inner.push(3);
    drop(inner);

    handle.join().unwrap();
    println!("{:?}", instance().lock().unwrap())
}


static GLOBAL: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();

fn instance() -> &'static Mutex<Vec<u8>> {
    GLOBAL.get_or_init(||{
        Mutex::new(vec![1])
    })
}

#[cfg(test)]
mod tests {
    use crate::once_call_test::test_once_call;

    #[test]
    fn test(){
        test_once_call();
    }
}