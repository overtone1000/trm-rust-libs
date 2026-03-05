type WebsocketError = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn get_next(websocket: HyperWebsocket) ->  Result<(), WebsocketError> {
    match websocket.await
    {
        Ok(websocket) => {
            while let Some(message) = websocket.next().await {
                Ok(message)
            }            
        },
        Err(e) => {
            Err(e)
        },
    }
}