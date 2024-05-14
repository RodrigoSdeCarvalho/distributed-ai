use url::Url;
use tungstenite::{connect, Message};
use std::thread;
use std::thread::sleep;
use chrono;
use rocket::serde::json::serde_json;
use serde::{Serialize, Deserialize};

use system::{Logger};

use super::data::SensorDataset;

pub struct Sensor {
    pub name: String,
    pub source: String,
    pub sensor_type: String,
    pub sampling_rate: i32, // in milliseconds
}

#[derive(Serialize, Deserialize, Debug)]
struct SensorDatapoint {
    pub source: String,
    pub sensor_type: String,
    pub timestamp: i64,
    pub value: f32,
}

pub const GLOBAL_SAMPLING_RATE: i32 = 5000; // 5 seconds in milliseconds

fn get_sensors() -> Vec<Sensor> {
    let sensors = vec!(Sensor {
        name: String::from("Kitchen_Brightness"),
        source: String::from("Kitchen"),
        sensor_type: String::from("Brightness"),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: String::from("Kitchen_Temperature"),
        source: String::from("Kitchen"),
        sensor_type: String::from("Temperature"),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: String::from("Kitchen_ThermostatTemperature"),
        source: String::from("Kitchen"),
        sensor_type: String::from("ThermostatTemperature"),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: String::from("Kitchen_Virtual_OutdoorTemperature"),
        source: String::from("Kitchen"),
        sensor_type: String::from("VirtualOutdoorTemperature"),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: String::from("Kitchen_Humidity"),
        source: String::from("Kitchen"),
        sensor_type: String::from("Humidity"),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },);
    sensors
}

impl Sensor {
    pub fn sense(
        self: &Self,
        gateway_url: impl AsRef<str>,
    ) {
        thread::sleep(std::time::Duration::from_secs(10));
        let (mut socket, response) = connect(
            Url::parse("ws://127.0.0.1:8000/echo").unwrap()
        ).expect("Can't connect");
        // Logger::info(
        //     &format!("Connected to websocket: {}", response.status()),
        //     true,
        // );

        // Logger::info(
        //     &format!("Sending data from sensor {}...", &self.name),
        //     true,
        // );
        let dataset = SensorDataset::new(&self.name);
        for value in dataset.iter() {
            let datapoint = SensorDatapoint {
                source: self.source.clone(),
                sensor_type: self.sensor_type.clone(),
                timestamp: chrono::Utc::now().timestamp(),
                value: value.unwrap(),
            };
            // Logger::info(
            //     &format!(
            //         "Sensed data from sensor {}: {} at {}",
            //         self.name, datapoint.value, datapoint.timestamp
            //     ),
            //     true,
            // );
            socket.send(Message::Text(
                serde_json::to_string(&datapoint).unwrap()
            )).unwrap();
            sleep(std::time::Duration::from_millis(1000));
        }
        socket.close(None).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sense() {
        let sensor = Sensor {
            name: "Kitchen_Brightness".to_string(),
            source: "Kitchen".to_string(),
            sensor_type: "Brightness".to_string(),
            sampling_rate: GLOBAL_SAMPLING_RATE,
        };
        sensor.sense("http://127.0.0.1:8000/ws");
    }

    #[test]
    fn test_get_sensors() {
        let (mut socket, response) = connect(
            Url::parse("ws://127.0.0.1:8000/echo").unwrap()
        ).expect("Can't connect");

        loop {
            print!("Enter a message: ");
            let message = socket.read().unwrap();
            println!("Received: {}", message);
        }
    }
}
