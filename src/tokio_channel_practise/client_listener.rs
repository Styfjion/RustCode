use crate::client::{Client, ClientMessage};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum Message {
    Receive {
        resp: oneshot::Sender<ClientMessage>,
        cancel_token: CancellationToken,
    },
    Send {
        message: ClientMessage,
        resp: oneshot::Sender<i32>,
    },
}

pub struct Listener {
    receive_channel: Receiver<Message>,
    client: Client,
}

impl Listener {
    pub fn new(receive_channel: Receiver<Message>) -> Self {
        Listener {
            receive_channel,
            client: Client::default(),
        }
    }

    pub async fn listen(&mut self) {
        while let Some(cmd) = self.receive_channel.recv().await {
            match cmd {
                Message::Receive { resp, cancel_token } => {
                    let receive_msg = self.client.receive(cancel_token).await;
                    // 忽略发送结果
                    let _ = resp.send(receive_msg);
                }
                Message::Send { message, resp } => {
                    let send_result = self.client.send_msg(message).await;
                    let _ = resp.send(send_result);
                }
            }
        }
    }
}
