use system::Logger;
use zeromq::subscriber;

fn main() {
    let sub_context = zmq::Context::new();
    Logger::trace("Starting Subscriber", true);
    let subs = subscriber::Subscriber::new(
        "Subscriber".to_string(),
        sub_context,
        "tcp://localhost:5555".to_string(),
        "tcp://localhost:5556".to_string(),
    );
    Logger::trace("Subscriber started", true);

    let binding = subs.receive();
    let message = binding
        .as_str()
        .unwrap();
    Logger::trace(&format!("Received: {}", message), true);
}