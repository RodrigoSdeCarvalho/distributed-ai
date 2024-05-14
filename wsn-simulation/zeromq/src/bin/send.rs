use system::Logger;
use zeromq::publisher;

fn main() {
    Logger::trace("Starting ZeroMQ publisher", true);
    let pub_context = zmq::Context::new();
    let publ = publisher::Publisher::new(
        "Publisher".to_string(),
        pub_context,
        "tcp://*:5555".to_string(),
        "tcp://*:5556".to_string(),
    );
    Logger::trace("Publisher started", true);

    Logger::trace("Waiting for subscribers", true);
    publ.receive_sync();
    Logger::trace("Subscriber connected", true);
    publ.send_sync();
    Logger::trace("Sync sent", true);

    publ.send("Hello World");
}