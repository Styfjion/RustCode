use std::borrow::Borrow;
use std::future::{Future, poll_fn};
use std::ops::Deref;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use common_exception::Result;


pub struct WrapperCancelToken {
    cancel_token: CancellationToken,
    register_action: Arc<dyn Future<Output=Result<()>> + Send + 'static + Sync>
}


impl Default for WrapperCancelToken {
    fn default() -> Self {
        WrapperCancelToken {
            cancel_token: CancellationToken::new(),
            register_action: Arc::new(async move {
                println!("No action");
                Ok(())
            })
        }
    }
}


impl WrapperCancelToken {

    pub fn register_action(&mut self, action: impl Future<Output=Result<()>> + Send + 'static + Sync) {
        self.register_action = Arc::new(action);
    }

    pub async fn cancel(&self) {
        self.cancel_token.cancel();
        tokio::spawn(self.register_action.deref().await);
    }

    pub fn inner(&self) -> CancellationToken {
        self.cancel_token.clone()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use crate::utils::wrapper_cancel_token::WrapperCancelToken;

    #[tokio::test]
    async fn test_fun() {
        let wrapper_token = WrapperCancelToken::default();
        let _ = wrapper_token.register_action.deref()().await;
    }
}

