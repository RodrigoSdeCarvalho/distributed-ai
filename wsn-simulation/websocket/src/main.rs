#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use sensors::sensor::SensorDatapoint;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref AGGREGATOR: Mutex<Aggregator> = Mutex::new(Aggregator {data: Vec::new()});
}

#[derive(Serialize)]
struct ComposeSensorData {
    items: Vec<SensorDatapoint>
}

#[derive(Serialize)]
struct StatusMessage {
    message: String
}

struct Aggregator {
    data: Vec<SensorDatapoint>
}

impl Aggregator {
    fn add_data(&mut self, data: SensorDatapoint) {
        self.data.push(data);
        if self.data.len() == 5 {
            self.send_batch();
            self.data.clear();
        }
    }

    fn send_batch(&self) {
        println!("Sending batch of data: {:?}", self.data);
    }

}

#[post("/push_data", format = "json", data = "<item>")]
fn push_data(item: Json<SensorDatapoint>) -> Result<Json<StatusMessage>, String> {
    // Extract the SensorDataPoint from the Json wrapper
    let sensor_data_point: SensorDatapoint = item.into_inner();

    AGGREGATOR.lock().unwrap().add_data(sensor_data_point);

    Ok(Json(StatusMessage {message: "Received and processed data successfully.".to_string()}))
}


#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, push_data])
}
