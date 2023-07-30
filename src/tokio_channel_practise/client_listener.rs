use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use crate::client::{Client, ClientMessage};

#[derive(Debug)]
pub enum Message {
    Receive {
        resp: oneshot::Sender<ClientMessage>,
    },
    Send {
        message: ClientMessage,
        resp: oneshot::Sender<i32>,
    },
}

pub struct Listener {
    
}

impl Listener {
    pub async fn listen(mut receive_channel: Receiver<Message>) {
        let mut client = Client::default();
        while let Some(cmd) = receive_channel.recv().await {
            match cmd {
                Message::Receive { resp } => {
                    let receive_msg = client.receive().await;
                    // 忽略发送结果
                    let _ = resp.send(receive_msg);
                }
                Message::Send { message, resp } => {
                    let send_result = client.send_msg(message).await;
                    let _ = resp.send(send_result);
                }
            }
        }
    }
}