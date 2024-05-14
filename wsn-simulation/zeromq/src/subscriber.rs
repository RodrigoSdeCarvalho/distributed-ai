use std::thread;
use std::time::Duration;
use zmq::Message;
use ::{zmq};

use system::{Logger};

pub struct Subscriber {
    pub name: String,
    pub context: zmq::Context,
    pub sync_service_url: String,
    pub sub_service_url: String,
    pub sub_service: SubService,
    pub sync_service: SyncService,
}

struct SubService {
    pub name: String,
    pub sub_service_url: String,
    pub actor: zmq::Socket,
}

struct SyncService {
    pub name: String,
    pub sync_service_url: String,
    pub actor: zmq::Socket,
}

impl SubService {
    pub fn new<'a>(name: String, context: &zmq::Context, sub_service_url: String) -> Self {
        let actor = context.socket(zmq::SUB).unwrap();
        actor
            .connect(sub_service_url.as_str())
            .expect("failed connecting Subscriber");
        actor
            .set_subscribe(b"")
            .expect("failed setting subscription");
        Self {
            name,
            sub_service_url,
            actor,
        }
    }
}

impl SyncService {
    pub fn new(name: String, context: &zmq::Context, sync_service_url: String) -> Self {
        let actor = context.socket(zmq::REQ).unwrap();
        actor
            .connect(sync_service_url.as_str())
            .expect("failed connecting SyncService");
        actor.send("", 0).expect("failed sending sync request");
        actor.recv_msg(0).expect("failed receiving sync reply");
        Self {
            name,
            sync_service_url,
            actor,
        }
    }
}

impl Subscriber {
    pub fn new(
        name: String,
        context: zmq::Context,
        sync_service_url: String,
        sub_service_url: String,
    ) -> Self {
        let name_clone1 = name.clone();
        let name_clone2 = name.clone();
        let sync_service_url_clone = sync_service_url.clone();
        let sub_service_url_clone = sub_service_url.clone();

        let sub_service = SubService::new(name_clone1, &context, sync_service_url_clone);
        thread::sleep(Duration::from_millis(1));
        let sync_service = SyncService::new(name_clone2, &context, sub_service_url_clone);

        Self {
            name,
            context,
            sync_service_url,
            sub_service_url,
            sub_service,
            sync_service,
        }
    }

    pub fn receive(&self) -> Message {
        let message = self.sub_service.actor.recv_msg(0).expect("failed receiving message");
        Logger::info(&format!("{} received: {:?}", self.name, message), true);
        message
    }
}
