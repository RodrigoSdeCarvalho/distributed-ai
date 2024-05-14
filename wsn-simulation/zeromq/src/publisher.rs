
use chrono;
use ::{zmq};

use system::{Logger};

pub struct Publisher {
    pub name: String,
    pub context: zmq::Context,
    pub sync_service_url: String,
    pub pub_service_url: String,
    pub publ_service: PubService,
    pub sync_service: SyncService,
}

struct PubService {
    pub name: String,
    pub pub_service_url: String,
    pub actor: zmq::Socket,
}

struct SyncService {
    pub name: String,
    pub sync_service_url: String,
    pub actor: zmq::Socket,
}

impl PubService {
    pub fn new(name: String, context: &zmq::Context, pub_service_url: String) -> Self {
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

impl SyncService {
    pub fn new(name: String, context: &zmq::Context, sync_service_url: String) -> Self {
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

impl Publisher {
    pub fn new(
        name: String,
        context: zmq::Context,
        sync_service_url: String,
        pub_service_url: String,
    ) -> Self {
        let name_clone1 = name.clone();
        let name_clone2 = name.clone();
        let sync_service_url_clone = sync_service_url.clone();
        let pub_service_url_clone = pub_service_url.clone();

        let publ_service = PubService::new(name_clone1.clone(), &context, pub_service_url_clone.clone());
        let sync_service = SyncService::new(name_clone2.clone(), &context, sync_service_url_clone.clone());
        Self {
            name,
            context,
            sync_service_url,
            pub_service_url,
            publ_service,
            sync_service,
        }
    }

    pub fn send(&self, message: &str) {
        self.publ_service.actor.send(message, 0).expect("failed broadcasting");
    }

    pub fn receive_sync(&self) {
        self.sync_service.actor.recv_msg(0).expect("failed receiving sync");
    }

    pub fn send_sync(&self) {
        self.sync_service.actor.send("", 0).expect("failed sending sync");
    }
}