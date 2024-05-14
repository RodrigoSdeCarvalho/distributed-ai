use system::Logger;
use zeromq::{publisher, subscriber};

fn main() {
    Logger::trace("Starting ZeroMQ publisher and subscriber", true);
    let pub_context = zmq::Context::new();
    let mut publ = publisher::Publisher::new(
        "Publisher".to_string(),
        pub_context,
        "tcp://*:5555".to_string(),
        "tcp://*:5556".to_string(),
    );
    Logger::trace("Publisher started", true);

    let sub_context = zmq::Context::new();
    Logger::trace("Starting Subscriber", true);
    let subs = subscriber::Subscriber::new(
        "Subscriber".to_string(),
        sub_context,
        "tcp://localhost:5555".to_string(),
        "tcp://localhost:5556".to_string(),
    );
    Logger::trace("Subscriber started", true);

    Logger::trace("Waiting for subscribers", true);

    publ.receive_sync();
    publ.send_sync();

    publ.send("Hello World");
    let binding = subs.receive();
    let message = binding
        .as_str()
        .unwrap();
    Logger::trace(&format!("Received: {}", message), true);
}

#[cfg(test)]
mod tests {
    use zeromq::publisher;
    use zeromq::subscriber;
    use system::Logger;

    // Make publisher and subscriber are working
    #[test]
    fn test_pub_sub() {
        let pub_context = zmq::Context::new();
        let mut publ = publisher::Publisher::new(
            "Publisher".to_string(),
            pub_context,
            "tcp://*:5555".to_string(),
            "tcp://*:5556".to_string(),
        );

        let sub_context = zmq::Context::new();
        let subs = subscriber::Subscriber::new(
            "Subscriber".to_string(),
            sub_context,
            "tcp://localhost:5555".to_string(),
            "tcp://localhost:5556".to_string(),
        );

        Logger::trace("Waiting for subscribers", true);
        for _ in 0..10 {
            publ.receive_sync();
            publ.send_sync();
        }

        publ.send("Hello World");
        let binding = subs.receive();
        let message = binding
            .as_str()
            .unwrap();
        Logger::trace(&format!("Received: {}", message), true);

        assert_eq!(message, "Hello World");
    }
}