use anyhow::{Error, Result};
use blake3::{Hash, Hasher};
use futures::{SinkExt, StreamExt};
use rayon::prelude::*;
use std::thread;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to {addr}");

    // tokio task和thread之间的通道
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用thread处理计算密集型任务
    thread::spawn(move || {
        while let Some((line, reply)) = receiver.blocking_recv() {
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash {hash}, nonce: {nonce}"),
                None => "Not found".to_string(),
            };

            if let Err(e) = reply.send(result) {
                eprintln!("Failed to send: {e}");
            }
        }
    });

    // 使用tokio 处理IO密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted {addr}");

        let sender1 = sender.clone();
        tokio::spawn(async move {
            let frame = Framed::new(stream, LinesCodec::new());
            let (mut w, mut r) = frame.split();
            while let Some(Ok(line)) = r.next().await {
                // 为每个消息创建一个 oneshot channel，用于发送回复
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line, reply))?;
                if let Ok(v) = reply_receiver.await {
                    // 发送计算完毕的结果
                    w.send(format!("Pow calculated: {v}")).await?;
                }
            }
            Ok::<_, Error>(())
        });
    }
}

pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

fn blake3_hash(mut hasher: Hasher, nonce: &u32) -> Hash {
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize()
}

fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hahser = Hasher::new();
    hahser.update(data);
    hahser
}
