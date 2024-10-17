use diesel::Connection;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub trait DatabaseTransactable<U>
where
    U: Connection,
{
    fn handle(self: Self, conn: &mut U);
}

pub struct AsyncDatabaseTransactionHandler<T, U>
where
    T: DatabaseTransactable<U>,
    U: Connection,
{
    conn: U,
    tx: UnboundedSender<T>,
    rx: UnboundedReceiver<T>,
}

impl<T, U> AsyncDatabaseTransactionHandler<T, U>
where
    T: DatabaseTransactable<U>,
    U: Connection,
{
    pub fn new(conn: U) -> AsyncDatabaseTransactionHandler<T, U> {
        let (tx, rx) = mpsc::unbounded_channel::<T>();
        AsyncDatabaseTransactionHandler {
            conn: conn,
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
                    println!("Received transaction. Sending to handler.");
                    transaction.handle(&mut self.conn);
                }
                None => {
                    //Do nothing
                }
            }
        }
    }
}
