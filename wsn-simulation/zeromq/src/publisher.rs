
use chrono;
use ::{zmq};

use system::{Logger};

//The publisher must have a socket to talk to clients and a socket to receive signals
struct Publisher {
    pub name: String,
    pub source: String,
    pub context: zmq::Context,
    pub sync_service_url: String,
    pub pub_service_url: String,
    pub sampling_rate: i32, // in milliseconds
    publ_service: PubService,
    subs_service: SubsService,
}

struct PubService {
    pub name: String,
    pub pub_service_url: String,
    pub actor: zmq::Socket, // not sure this is the type
}

struct SubsService {
    pub name: String,
    pub sync_service_url: String,
    pub actor: zmq::Socket, // not sure this is the type
}

impl PubService {
    pub fn new(name: String, context: zmq::Context, pub_service_url: String) -> Self {
        let actor = context.socket(zmq::PUB).unwrap();
        actor.set_sndhwm(1_100_000).expect("failed setting hwm");
        actor
            .bind(pub_service_url.as_str())
            .expect("failed binding Publisher");
        Self {
            name,
            pub_service_url,
            actor,
        }
    }
}

impl SubsService {
    pub fn new(name: String, context: zmq::Context, sync_service_url: String) -> Self {
        let actor = context.socket(zmq::REP).unwrap();
        actor
            .bind(sync_service_url.as_str())
            .expect("failed binding SubsService");
        Self {
            name,
            sync_service_url,
            actor,
        }
    }
}

//In the publisher implementation, in the constructor we build the publisher and the subscriber we also have send and receive method
impl Publisher {
    pub fn new(
        name: String,
        source: String,
        context: zmq::Context::new(),
        sync_service_url: String,
        pub_service_url: String,
        sampling_rate: i32,
    ) -> Self {
        Self {
            name,
            source,
            context,
            sync_service_url,
            pub_service_url,
            sampling_rate,
            publ_service: PubService::new(name.clone(), context.clone(), pub_service_url.clone()),
            subs_service: SubsService::new(name.clone(), context.clone(), sync_service_url.clone()),
        }
    }

    pub fn send(&self, message: &str) {
        self.publ_service.actor.send(message, 0).expect("failed broadcasting");
    }

    pub fn receive(&self) {
        self.subs_service.actor.recv_msg(0).expect("failed receiving sync");
    }
}