use tungstenite::{connect, Message};
use url::Url;

fn main() {
    println!("Enter a message: ");
    let (mut socket, response) = connect(
        Url::parse("ws://127.0.0.1:8000/echo").unwrap()
    ).expect("Can't connect");

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
