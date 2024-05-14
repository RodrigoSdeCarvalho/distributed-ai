use zeromq::publisher;
//call publisher and subscriber in the main function
fn main() {
}

#[cfg(test)]
mod tests {
    use zeromq::publisher;
    use zeromq::subscriber;
    use super::*;

    // Make publisher and subscriber are working
    #[test]
    fn test_pub_sub() {
        let context = zmq::Context::new();

        let mut publ = publisher::Publisher::new(
            "Publisher".to_string(),
            &context,
            "tcp://*:5555".to_string(),
            "tcp://*:5556".to_string(),
        );

        let mut subs = subscriber::Subscriber::new(
            "Subscriber".to_string(),
            &context,
            "tcp://localhost:5555".to_string(),
            "tcp://localhost:5556".to_string(),
        );

        println!("Waiting for subscribers");
        for _ in 0..10 {
            publ.receive_sync();
            publ.send_sync();
        }

        publ.send(&"Hello World".to_string());
        let message = subs.receive();
        assert_eq!(message, "Hello World");
    }
}