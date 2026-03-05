use hyper_tungstenite::{HyperWebsocket, WebSocketStream, hyper::upgrade::Upgraded, tungstenite::Message};
use hyper_util::rt::TokioIo;

type WebsocketError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct WebSocketStreamNext
{
    websocket_stream:WebSocketStream<TokioIo<Upgraded>>,
    message:Message
}

impl WebSocketStreamNext
{
    pub async fn get_next(websocket: HyperWebsocket) ->  Result<WebSocketStreamNext, WebsocketError> {
        match websocket.await
        {
            Ok(mut websocket_stream) => {
                match futures_util::StreamExt::next(&mut websocket_stream).await {
                    Some(message) => {
                        match message {
                            Ok(message) => Ok(
                                WebSocketStreamNext
                                {
                                    websocket_stream,
                                    message
                                }
                            ),
                            Err(e) => Err(Box::new(e)),
                        }
                    },
                    None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"No message."))),
                }
            },
            Err(e) => {
                Err(Box::new(e))
            },
        }
    }

    pub fn get_message(&self)->&Message
    {
        &self.message
    }

    pub async fn send_message(&mut self, message:Message) -> Result<(), WebsocketError>
    {
        match futures_util::SinkExt::send(&mut self.websocket_stream, message).await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }
}