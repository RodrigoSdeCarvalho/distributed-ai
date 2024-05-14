use std::thread;

use sensors::sensor::{GLOBAL_SAMPLING_RATE, Sensor, get_sensors};

#[tokio::main]
async fn main() {
    let sensors = get_sensors();
    println!("{}", sensors.len());
    for sensor in sensors {
        sensor.sense(String::from("http://127.0.0.1:8000/push_data")).await;
        println!("Starting sensor: {}", sensor.name);
    }
}