use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async  fn main() {
    // let url = "wss://echo.websocket.events";
    let url = "ws://localhost:3000";

    println!("Connecting to - {}",url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connecting");
    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();

    let bot_name = "RustTimeBot";
    let msg = Message::Text(format!("register as {}", bot_name).into());
    println!("sending message: {}", msg);
    write.send(msg).await.expect("Failed to send message");

    let msg = Message::Text("who's connected?".into());
    println!("sending message: {}", msg);
    write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read the message");
        println!("Received a message: {}", message);
    }
}
