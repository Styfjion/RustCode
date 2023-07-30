use crate::client::ClientMessage;
use crate::client_listener::Message;
use crate::StdError;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

pub struct Task {}
impl Task {
    pub async fn execute_task(
        message: ClientMessage,
        cancel_token: CancellationToken,
    ) -> Result<(), StdError> {
        println!("start task, start message is {:?}", message);
        //模拟执行任务，收到cancel信号后停止任务
        select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                println!("task has been done");
            },
            _ = cancel_token.cancelled() => {
                println!("task has been cancelled from job, task is stopped")
            }
        }
        Ok(())
    }

    pub async fn receive_message(tx: mpsc::Sender<Message>) -> Result<ClientMessage, StdError> {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Message::Receive { resp: resp_tx }).await?;
        let msg = resp_rx.await?;
        println!("task receive msg is {:?}", msg);
        Ok(msg)
    }

    pub async fn send_result(tx: mpsc::Sender<Message>, message: ClientMessage) -> Result<(), StdError> {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Message::Send {
            message,
            resp: resp_tx,
        })
        .await?;
        let response_code = resp_rx.await?;
        println!("send task response code is {response_code}");
        Ok(())
    }
}
