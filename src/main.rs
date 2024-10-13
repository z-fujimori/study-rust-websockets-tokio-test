use futures_util::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

async fn register_bot(write: &mut SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>, bot_name: &str) {
    let registration_message = Message::Text(format!("register as {}", bot_name));
    write.send(registration_message).await.expect("Failed to send registration message");
}

async fn handle_incoming_messages(mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>) {
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}

#[tokio::main]
async  fn main() {
    let url = "wss://echo.websocket.events";
    // let url = "ws://localhost:3000";

    println!("Connecting to - {}",url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connecting");
    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();

    // register the timebot
    register_bot(&mut write, "RustTimeBot").await;
    // let bot_name = "RustTimeBot";
    // let msg = Message::Text(format!("register as {}", bot_name).into());
    // println!("sending message: {}", msg);
    // write.send(msg).await.expect("Failed to send message");

    let msg = Message::Text("who's connected?".into());
    println!("sending message: {}", msg);
    write.send(msg).await.expect("Failed to send message");

    let read_handle = tokio::spawn(handle_incoming_messages(read));
    // 19:12
    // if let Some(message) = read.next().await{
    //     let message = message.expect("Failed to read the message");
    //     println!("Received a message: {}", message);
    // }
    let _ = tokio::try_join!(read_handle);
}
