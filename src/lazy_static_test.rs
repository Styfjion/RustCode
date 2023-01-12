use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

pub fn test_lazy() {
    do_a_call();
    let task_1 = thread::spawn(do_a_call);
    let task_2 = thread::spawn(do_a_call);
    task_1.join().unwrap();
    task_2.join().unwrap();
    println!("called {}", ARRAY.lock().unwrap().len());
}

#[cfg(test)]
mod tests {
    use crate::lazy_static_test::test_lazy;

    #[test]
    fn test() {
        test_lazy();
    }
}