#[macro_use] extern crate rocket;

use rocket::futures::{SinkExt, StreamExt};

#[get("/echo")]
fn echo(ws: ws::WebSocket) -> ws::Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            println!("{:?}", message);
            let _ = stream.send(message?).await;
        }

        Ok(())
    }))
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![echo])
}
