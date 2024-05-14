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
        let pub_context = zmq::Context::new();
        let mut publ = publisher::Publisher::new(
            "Publisher".to_string(),
            pub_context,
            "tcp://*:5555".to_string(),
            "tcp://*:5556".to_string(),
        );

        let sub_context = zmq::Context::new();
        let mut subs = subscriber::Subscriber::new(
            "Subscriber".to_string(),
            sub_context,
            "tcp://localhost:5555".to_string(),
            "tcp://localhost:5556".to_string(),
        );

        println!("Waiting for subscribers");
        for _ in 0..10 {
            publ.receive_sync();
            publ.send_sync();
        }

        publ.send(&"Hello World".to_string());
        let message: &str = subs.receive();
        assert_eq!(message, &"Hello World".to_string());
    }
}