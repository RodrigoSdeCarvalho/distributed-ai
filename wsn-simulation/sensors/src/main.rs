use std::thread;
use rocket::log::private::info;

use sensors::sensor::{GLOBAL_SAMPLING_RATE, Sensor, get_sensors};
use system::Logger;

#[tokio::main]
async fn main() {
    let sensors = get_sensors();
    for sensor in sensors {
        Logger::info(format!("Starting sensor: {}", sensor.name), true);
        tokio::spawn(async move {
            sensor.sense(String::from("http://127.0.0.1:8000/push_data")).await;
        });
    }
}