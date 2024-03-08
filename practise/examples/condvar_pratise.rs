use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("child thread get lock");
        drop(started);
        println!("child thread unlocked");
        // 通知主线程
        cvar.notify_one();
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("working...");   
        }
    });

    // 等待工作线程的通知
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("main thread get lock");
    while !*started {
        println!("main thread release lock to wait");
        started = cvar.wait(started).unwrap();
    }
    println!("Worker started!");
    thread::sleep(Duration::from_secs(2));
    println!("end");
}

// main thread get lock
// main thread release lock to wait
// child thread get lock
// child thread unlocked
// Worker started!
// working...
// end
// working...