use chrono::{DateTime, Local};
use std::fs::read;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum ClientMessage {
    Start {
        id: i32,
        name: String,
        time: DateTime<Local>,
    },
    Cancel {
        id: i32,
    },
    Result {
        id: i32,
        status: String,
        code: i32,
    },
}

#[derive(Default)]
pub struct Client {
    cnt: i32,
}

impl Client {
    pub async fn send_msg(&self, message: ClientMessage) -> i32 {
        // 模拟发送过程
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!(
            "client {} Send message success, message is {:?}",
            self.cnt, message
        );
        0
    }

    pub async fn receive(&mut self, cancel_token: CancellationToken) -> ClientMessage {
        tokio::select! {
            result = self.receive_impl() => result,
            _ = cancel_token.cancelled() => {
                println!("channel listener has been cancelled from job, listening is stopped");
                ClientMessage::Cancel{id: 0}
            }
        }
    }

    async fn receive_impl(&mut self) -> ClientMessage {
        if self.cnt == 0 {
            self.cnt += 1;
            ClientMessage::Start {
                id: self.cnt,
                name: "job_1".to_string(),
                time: Local::now(),
            }
        } else {
            //模拟监听取消
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            ClientMessage::Cancel { id: self.cnt }
        }
    }
}
