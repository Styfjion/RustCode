use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait OutputReceiver {
    type Error;
    async fn run_async(&mut self) -> Result<(), Self::Error>;
    async fn set_error(&mut self, err_msg: String);
}

#[derive(Clone)]
pub struct DefaultOutputReceiver {
    pub output_file: PathBuf,
    pub output_content: Arc<Mutex<String>>,
}

#[async_trait]
impl OutputReceiver for DefaultOutputReceiver {
    type Error = Box<dyn std::error::Error>;
    async fn run_async(&mut self) -> Result<(), Self::Error> {
        let mut output_content = self.output_content.lock().await;
        *output_content = tokio::fs::read_to_string(&self.output_file).await?;
        Ok(())
    }

    async fn set_error(&mut self, err_msg: String) {
        let mut output_content = self.output_content.lock().await;
        *output_content = err_msg;
    }
}
