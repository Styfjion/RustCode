use tokio_util::sync::CancellationToken;

#[tonic::async_trait]
pub trait AsyncCallback {
    async fn execute_call_back(&self);
}

struct DummyAction {

}

#[tonic::async_trait]
impl AsyncCallback for DummyAction {
    async fn execute_call_back(&self) {
        println!("No action");
    }
}

pub struct WrapperCancelToken {
    cancel_token: CancellationToken,
    action: Box<dyn AsyncCallback + Send + Sync + 'static>,
}

impl Clone for WrapperCancelToken {
    fn clone(&self) -> Self {
        WrapperCancelToken {
            cancel_token: self.cancel_token.clone(),
            action: Box::new(DummyAction{}),
        }
    }
}

impl Default for WrapperCancelToken {
    fn default() -> Self {
        WrapperCancelToken {
            cancel_token: CancellationToken::new(),
            action: Box::new(DummyAction{}),
        }
    }
}


impl WrapperCancelToken {

    pub fn register_action(&mut self, action: impl AsyncCallback + Send + Sync + 'static) {
        self.action = Box::new(action);
    }

    pub async fn cancel_with_post_action(&self) {
        self.cancel_token.cancel();
        self.action.execute_call_back().await;
    }

    pub fn inner(&self) -> CancellationToken {
        self.cancel_token.clone()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::select;
    use tokio::sync::Mutex;
    use crate::utils::wrapper_cancel_token::{AsyncCallback, WrapperCancelToken};

    #[tokio::test]
    async fn test_init_func() {
        let wrapper_token = WrapperCancelToken::default();
        wrapper_token.cancel_with_post_action().await;
    }

    struct WrapperData {
        data: Arc<Mutex<HashMap<i32, i32>>>
    }

    #[tonic::async_trait]
    impl AsyncCallback for WrapperData {
        async fn execute_call_back(&self) {
            let mut data = self.data.lock().await;
            println!("Task has been cancelled");
            data.insert(2, 2);
            println!("data is {:?}", data);
        }
    }

    #[tokio::test]
    async fn test_register() {
        let mut wrapper_token = WrapperCancelToken::default();
        let wrapper_data = WrapperData{data:Arc::new(Mutex::new(HashMap::from([(1, 1)])))};
        wrapper_token.register_action(wrapper_data);
        let clone_token = wrapper_token.inner().clone();

        let join_handle = tokio::spawn(async move {
            select! {
                _ = clone_token.cancelled() => {5}
                _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {99}
            }
        });

        let cancel_handle = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            wrapper_token.cancel_with_post_action().await;
        });

        assert_eq!(5, join_handle.await.unwrap());
        cancel_handle.await.unwrap();
    }
}

