mod client;
mod client_listener;
mod task;

use crate::client::ClientMessage;
use crate::client_listener::Listener;
use crate::task::Task;
use chrono::Local;
use tokio::select;
use tokio_util::sync::CancellationToken;

type StdError = Box<dyn std::error::Error + Send + Sync>;

async fn execute_job(task_secs: u64) -> Result<(), StdError> {
    // 准备通道
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let mut listener = Listener::new(rx);

    // 独立启动客户端，开始监听
    let listener = tokio::spawn(async move { listener.listen().await });
    let tx_2 = tx.clone();
    let tx_3 = tx.clone();

    // 接收启动消息
    let start_msg = Task::receive_message(tx, CancellationToken::new()).await?;

    let start_time = Local::now();
    // 开始执行任务
    let cancel_token = CancellationToken::new();
    let cloned_token = cancel_token.clone();
    let mut task =
        tokio::spawn(async move { Task::execute_task(start_msg, cloned_token, task_secs).await });

    // 监听是否收到取消消息
    let channel_token = CancellationToken::new();
    let cloned_channel_token = channel_token.clone();
    let cancel_task =
        tokio::spawn(async move { Task::receive_message(tx_2, cloned_channel_token).await });

    select! {
        result = &mut task => {
            match result {
                Ok(task_val) => {
                    match task_val {
                        Ok(val) => {
                            println!("job has been done, val is {:?}", val);
                            // 发送结果消息
                            channel_token.cancel();
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
    println!(
        "program has been finished, time use is {}s",
        (Local::now() - start_time).num_seconds()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), StdError> {
    execute_job(1).await?;
    println!("------------------------------");
    execute_job(10).await?;
    Ok(())
}

/*
    task receive msg is Start { id: 1, name: "job_1", time: 2023-07-31T19:03:26.337721500+08:00 }
    start task, start message is Start { id: 1, name: "job_1", time: 2023-07-31T19:03:26.337721500+08:00 }
    task has been done
    job has been done, val is "Success"
    channel listener has been cancelled from job, listening is stopped
    task receive msg is Cancel { id: 0 }
    client 1 Send message success, message is Result { id: 1, status: "Success", code: 0 }
    send task response code is 0
    program has been finished, time use is 2s
    ------------------------------
    task receive msg is Start { id: 1, name: "job_1", time: 2023-07-31T19:03:28.357323600+08:00 }
    start task, start message is Start { id: 1, name: "job_1", time: 2023-07-31T19:03:28.357323600+08:00 }
    task receive msg is Cancel { id: 1 }
    job has received cancel message is Cancel { id: 1 }, begin stop task
    task has been cancelled from job, task is stopped
    client 1 Send message success, message is Result { id: 1, status: "Cancelled", code: 1 }
    send task response code is 0
    program has been finished, time use is 6s
*/
