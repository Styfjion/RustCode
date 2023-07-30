mod client;
mod client_listener;
mod task;

use crate::client::ClientMessage;
use crate::client_listener::Listener;
use crate::task::Task;
use tokio::select;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

type StdError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), StdError> {
    // 准备通道
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let tx_2 = tx.clone();
    let tx_3 = tx.clone();

    // 独立启动客户端，开始监听
    let listener = tokio::spawn(async move { Listener::listen(rx).await });

    // 接收启动消息
    let start_msg = Task::receive_message(tx).await?;

    // 开始执行任务
    let cancel_token = CancellationToken::new();
    let cloned_token = cancel_token.clone();
    let mut task = tokio::spawn(async move { Task::execute_task(start_msg, cloned_token).await });

    // 监听是否收到取消消息
    let cancel_task: JoinHandle<Result<ClientMessage, StdError>> =
        tokio::spawn(async move { Task::receive_message(tx_2).await });

    select! {
        result = &mut task => {
            match result {
                Ok(task_val) => {
                    match task_val {
                        Ok(val) => {
                            println!("job has been done, val is {:?}", val);
                            // 发送结果消息
                            cancel_token.cancel();
                            Task::send_result(tx_3, ClientMessage::Result {id: 1, status: "Success".to_string(), code: 0}).await?;
                        },
                        Err(error) => eprintln!("job task error is {:?}", error)
                    }
                },
                Err(error) => eprintln!("wait job task error is {:?}", error)
            }
        },
        result = cancel_task => {
            match result {
                Ok(cancel_task_val) => {
                    match cancel_task_val {
                        Ok(val) => {
                            println!("job has received cancel message is {:?}, begin stop task", val);
                            cancel_token.cancel();
                            let _ = task.await?;
                            // 发送结果消息
                            Task::send_result(tx_3, ClientMessage::Result {id: 1, status: "Cancelled".to_string(), code: 1}).await?;
                        },
                        Err(error) => eprintln!("job cancel task error is {:?}", error)
                    }
                },
                Err(error) => eprintln!("wait job cancel task error is {:?}", error)
            }
        }
    }
    listener.await?;
    println!("program has been finished");
    Ok(())
}

/*
    task receive msg is Start { id: 1, name: "job_1", time: 2023-07-30T18:22:56.972797+08:00 }
    start task, start message is Start { id: 1, name: "job_1", time: 2023-07-30T18:22:56.972797+08:00 }
    task receive msg is Cancel { id: 1 }
    job has received cancel message is Cancel { id: 1 }, begin stop task
    task has been cancelled from job, task is stopped
    client 1 Send message success, message is Result { id: 1, status: "Cancelled", code: 1 }
    send task response code is 0
    program has been finished
*/
