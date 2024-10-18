use std::thread::sleep;

use diesel::Connection;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub trait DatabaseTransactable<U>
where
    U: Connection,
{
    fn handle(self: Self, conn: &mut U);
}

pub struct AsyncDatabaseTransactionHandler<T, U, V>
where
    T: DatabaseTransactable<U>,
    U: Connection,
    V: Fn() -> U,
{
    conn: Option<U>,
    conn_builder: V,
    stopwatch: stopwatch::Stopwatch,
    tx: UnboundedSender<T>,
    rx: UnboundedReceiver<T>,
}

const CONNECTION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);
const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

impl<T, U, V> AsyncDatabaseTransactionHandler<T, U, V>
where
    T: DatabaseTransactable<U>,
    U: Connection,
    V: Fn() -> U,
{
    pub fn new(conn_builder: V) -> AsyncDatabaseTransactionHandler<T, U, V> {
        let (tx, rx) = mpsc::unbounded_channel::<T>();
        AsyncDatabaseTransactionHandler {
            conn: None,
            conn_builder: conn_builder,
            stopwatch: stopwatch::Stopwatch::new(),
            tx: tx,
            rx: rx,
        }
    }

    pub fn get_sender(&self) -> UnboundedSender<T> {
        self.tx.clone()
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            match self.rx.recv().await {
                Some(transaction) => {
                    self.stopwatch.reset();

                    println!("Received transaction. Sending to handler.");

                    if self.conn.is_none() {
                        self.conn = Some((self.conn_builder)());
                    }

                    let conn: &mut U = &mut (self
                        .conn
                        .as_mut()
                        .expect("Should have just been created if it was none before."));

                    transaction.handle(conn);
                }
                None => {
                    if self.conn.is_some() && self.stopwatch.elapsed() > CONNECTION_TIMEOUT {
                        self.conn = None;
                        self.stopwatch.reset();
                    } else if !self.stopwatch.is_running() {
                        self.stopwatch.start();
                    }
                    sleep(SLEEP_DURATION);
                }
            }
        }
    }
}
