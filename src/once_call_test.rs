// use std::{sync::OnceLock, thread};
// pub fn test_once_call() {
//     // 子线程中调用
//     let handle = thread::spawn(|| {
//         let logger = Logger::global();
//         logger.log("thread message".to_string());
//     });
//
//     // 主线程调用
//     let logger = Logger::global();
//     logger.log("some message".to_string());
//
//     let logger2 = Logger::global();
//     logger2.log("other message".to_string());
//
//     handle.join().unwrap();
// }
//
// #[derive(Debug)]
// struct Logger;
//
// static LOGGER: OnceLock<Logger> = OnceLock::new();
//
// impl Logger {
//     fn global() -> &'static Logger {
//         // 获取或初始化 Logger
//         LOGGER.get_or_init(|| {
//             println!("Logger is being created..."); // 初始化打印
//             Logger
//         })
//     }
//
//     fn log(&self, message: String) {
//         println!("{}", message)
//     }
// }
