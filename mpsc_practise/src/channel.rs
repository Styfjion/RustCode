use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use anyhow::{anyhow, Result};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

const INITIAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receivers left"));
        }

        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        if was_empty {
            self.shared.available.notify_one();
        }
        Ok(())
    }

    pub fn total_queued_items(&self) -> usize {
        self.shared.queue.lock().unwrap().len()
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if old <= 1 {
            self.shared.available.notify_all();
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: self.shared.clone(),
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(val) = self.cache.pop_front() {
            return Ok(val);
        }
        
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(val) => {
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    return Ok(val);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("no sender left")),
                None => {
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::channel::unbounded;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello, world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello, world!");
    }

    #[test]
    fn mutiple_senders_should_work() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }
        let mut result = r.into_iter().collect::<Vec<_>>();
        result.sort();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn receiver_should_be_vlocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                assert_eq!(idx, i);
            }
            panic!()
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        thread::sleep(Duration::from_millis(1));

        for i in 100..200usize {
            s1.send(i).unwrap();
        }
        thread::sleep(Duration::from_millis(1));

        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
            });
        }

        for _ in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        //用于线程同步,
        let (mut sender, mut receiver) = unbounded::<usize>();
        let t1 = thread::spawn(move || {
            // 保证 r.recv() 先于 t2 的 drop 执行
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
        });
        t1.join().unwrap();
    }
    
    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();
        for i in 0..10usize {
            s.send(i).unwrap();
        }
        
        assert!(r.cache.is_empty());
        assert_eq!(0, r.recv().unwrap());
        assert_eq!(r.cache.len(), 9);
        assert_eq!(s.total_queued_items(), 0);
        for (idx, i) in r.take(9).enumerate() {
            assert_eq!(idx + 1, i);
        }
    }
}
