use ureq;
use chrono;
use rocket::serde::json::serde_json;
use serde::{Serialize, Deserialize};

use super::data::SensorDataset;

pub struct Sensor {
    pub name: String,
    pub source: String,
    pub sensor_type: String,
    pub sampling_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SensorDatapoint {
    pub source: String,
    pub sensor_type: String,
    pub timestamp: i64,
    pub value: f32,
}

pub const GLOBAL_SAMPLING_RATE: i32 = 5000; // 5 seconds in milliseconds

pub fn get_sensors() -> Vec<Sensor> {
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

#[derive(serde::Deserialize, Debug)]
struct StatusMessage {
    status: String,
    message: String,
}

impl Sensor {
    pub fn sense(
        self: &Self,
        gateway_url: String,
    ) {
        // let (mut socket, response) = connect(
        //     Url::parse(gateway_url).unwrap()
        // ).expect("Can't connect");

        let dataset = SensorDataset::new(&self.name);

        for value in dataset.iter() {
            let datapoint = SensorDatapoint {
                source: self.source.clone(),
                sensor_type: self.sensor_type.clone(),
                timestamp: chrono::Utc::now().timestamp(),
                value: value.unwrap(),
            };

            let json_value = serde_json::to_value(datapoint).unwrap();

            let response: StatusMessage = ureq::post(gateway_url.as_str())
                .send_json(json_value).unwrap()
                .into_json().unwrap();

            println!("{:?}", response);

            // socket.write_message(Message::Text(
            //     serde_json::to_string(&datapoint).unwrap()
            // )).unwrap();
            // sleep(std::time::Duration::from_millis(1000));
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::de::Unexpected::Str;
    use super::*;

    #[test]
    fn test_sense() {
        let sensor = Sensor {
            name: "Kitchen_Brightness".to_string(),
            source: "Kitchen".to_string(),
            sensor_type: "Brightness".to_string(),
            sampling_rate: GLOBAL_SAMPLING_RATE,
        };
        sensor.sense(String::from("http://127.0.0.1:8000/push-data"));
    }
}
