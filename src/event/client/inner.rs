use reqwest::Client as ReqwestClient;
use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};

#[derive(Debug)]
pub struct Client<T> {
    inner: Option<ReqwestClient>,
    sender: Sender<T>,
    recver: Receiver<T>,
}

impl<T> Client<T> {
    pub fn new() -> Self {
        let (sender, recver) = channel(5);
        let inner = ReqwestClient::builder().build().ok();
        Self {
            inner,
            sender,
            recver,
        }
    }

    pub fn get_sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub fn immediate_recv(&mut self) -> Option<T> {
        match self.recver.try_recv() {
            Ok(res) => Some(res),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Sender should not have dropped"),
        }
    }

    pub fn get_inner(&self) -> &ReqwestClient {
        self.inner
            .as_ref()
            .expect("Reqwest Client should be built successfully")
    }

    pub fn is_invalid(&self) -> bool {
        self.inner.is_none()
    }
}

impl<T> Default for Client<T> {
    fn default() -> Self {
        Self::new()
    }
}
