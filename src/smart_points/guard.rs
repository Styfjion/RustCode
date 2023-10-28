use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
lazy_static! {
    // 一般情况下 Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
    // 如果你把 Mutex 声明成 static，其生命周期是静态的，不需要 Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> = Mutex::new(HashMap::new());
}

fn main() {
    for _ in 0..32 {
        thread::spawn(|| {
            let mut g = METRICS.lock().unwrap();
            *(g.deref_mut().entry("hello".into()).or_insert(0)) += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("metrics : {:?}", METRICS.lock().unwrap());
}
