use futures::{FutureExt, Stream, StreamExt};
use tokio::{
    sync::mpsc::{channel, error::TryRecvError, Receiver},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct Listener<T> {
    event_stream: Receiver<T>,
    handler: Option<JoinHandle<()>>,
}

impl<T> Listener<T>
where
    T: Send + 'static,
{
    pub fn new<S>(mut stream: S) -> Self
    where
        S: Stream + Unpin + Send + 'static,
        S::Item: Into<T> + Send,
    {
        let (sender, event_stream) = channel(10);
        let handler = tokio::spawn(async move {
            loop {
                let next = stream.next().fuse().await;
                match next {
                    Some(into_res) => {
                        let res = into_res.into();
                        sender
                            .send(res)
                            .await
                            .expect("Recver should not have dropped");
                    }
                    None => break,
                }
            }
        });
        Self {
            event_stream,
            handler: Some(handler),
        }
    }

    pub fn listen(&mut self) -> Option<T> {
        let event_res = self.event_stream.try_recv();
        match event_res {
            Ok(res) => Some(res),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Sender should not have dropped"),
        }
    }

    pub async fn listen_until(&mut self) -> T {
        self.event_stream
            .recv()
            .await
            .expect("Sender should not have been dropped")
    }
}

impl<T> Drop for Listener<T> {
    fn drop(&mut self) {
        let op_handler = self.handler.take();
        if let Some(handler) = op_handler {
            handler.abort();
        }
    }
}
