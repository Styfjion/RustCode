use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use crate::process_invoker::output_receiver::OutputReceiver;

type Error = Box<dyn std::error::Error>;
pub struct ProcessInvoker {
    stdout_receiver: Option<Box<dyn OutputReceiver<Error = Error>>>,
    stderr_receiver: Option<Box<dyn OutputReceiver<Error = Error>>>,
    child: Child,
}

impl ProcessInvoker {
    pub fn new(
        cmd: &str,
        args: Vec<String>,
        output_file: &PathBuf,
        err_file: &PathBuf,
    ) -> Result<Self, Error> {
        let mut cmd = Command::new(cmd);
        if !args.is_empty() {
            cmd.args(args);
        }
        let child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::from(File::create(output_file)?))
            .stderr(Stdio::from(File::create(err_file)?))
            .spawn()?;

        Ok(ProcessInvoker {
            stdout_receiver: None,
            stderr_receiver: None,
            child,
        })
    }

    pub fn set_out_receiver(&mut self, out_receiver: impl OutputReceiver<Error = Error> + 'static) {
        self.stdout_receiver = Some(Box::new(out_receiver));
    }

    pub fn set_err_receiver(&mut self, out_receiver: impl OutputReceiver<Error = Error> + 'static) {
        self.stderr_receiver = Some(Box::new(out_receiver));
    }

    pub async fn send_msg(&mut self, msg: &str) -> Result<(), Error> {
        if let Some(val) = &mut self.child.stdin {
            val.write(msg.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn wait_with_timeout(&mut self, timeout: u64) -> Result<(), Error> {
        match tokio::time::timeout(tokio::time::Duration::from_secs(timeout), self.child.wait())
            .await
        {
            Ok(val) => {
                let _ = val?;
                if let Some(mut output_receiver) = self.stdout_receiver.take() {
                    output_receiver.run_async().await?;
                }

                if let Some(mut errer_receiver) = self.stderr_receiver.take() {
                    errer_receiver.run_async().await?;
                }
            }
            Err(error) => {
                if let Some(mut error_receiver) = self.stderr_receiver.take() {
                    error_receiver.set_error(format!("error is {error}")).await;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::current_dir;
    use std::ops::Deref;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::process_invoker::output_receiver::DefaultOutputReceiver;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_without_timeout() -> Result<(), Error> {
        let output_file = PathBuf::from("output.txt");
        let error_file = PathBuf::from("error.txt");
        let mut process_invoker = ProcessInvoker::new("sort", vec![], &output_file, &error_file)?;
        let output_content = Arc::new(Mutex::new(String::new()));
        let error_content = Arc::new(Mutex::new(String::new()));
        let out_receiver = DefaultOutputReceiver {
            output_file,
            output_content: output_content.clone(),
        };
        let err_receiver = DefaultOutputReceiver {
            output_file: error_file,
            output_content: error_content.clone()
        };
        process_invoker.set_out_receiver(out_receiver);
        process_invoker.set_err_receiver(err_receiver);
        process_invoker.send_msg("2\n1\n4\n3").await?;
        process_invoker.wait_with_timeout(2).await?;
        let output_content_arc = output_content.lock().await;
        println!("{output_content_arc}");
        assert_eq!(output_content_arc.deref(), "1\n2\n3\n4\n");
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_with_timeout() -> Result<(), Error> {
        let output_file = PathBuf::from("output2.txt");
        let error_file = PathBuf::from("error2.txt");
        let mut process_invoker = ProcessInvoker::new(
            "sh",
            vec![current_dir()?.join("test.sh").to_string_lossy().to_string()],
            &output_file,
            &error_file,
        )?;
        let output_content = Arc::new(Mutex::new(String::new()));
        let error_content = Arc::new(Mutex::new(String::new()));
        let out_receiver = DefaultOutputReceiver {
            output_file,
            output_content: output_content.clone(),
        };
        let err_receiver = DefaultOutputReceiver {
            output_file: error_file,
            output_content: error_content.clone(),
        };
        process_invoker.set_out_receiver(out_receiver);
        process_invoker.set_err_receiver(err_receiver);
        process_invoker.wait_with_timeout(2).await?;
        let error_content_arc = error_content.lock().await;
        assert_eq!(error_content_arc.deref(), "error is deadline has elapsed");
        Ok(())
    }
}
