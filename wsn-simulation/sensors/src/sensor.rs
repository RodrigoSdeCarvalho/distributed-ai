use std::thread;
use chrono;

use system::{Logger};

use super::data::SensorDataset;

struct Sensor {
    pub name: String,
    pub source: String,
    pub sensor_type: String,
    pub sampling_rate: i32, // in milliseconds
}

struct SensorDatapoint {
    pub source: String,
    pub sensor_type: String,
    pub timestamp: i64,
    pub value: f32,
}

const GLOBAL_SAMPLING_RATE: i32 = 5000; // 5 seconds in milliseconds

const SENSORS: [Sensor; 5] = [
    Sensor {
        name: "Kitchen_Brightness".to_string(),
        source: "Kitchen".to_string(),
        sensor_type: "Brightness".to_string(),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: "Kitchen_Temperature".to_string(),
        source: "Kitchen".to_string(),
        sensor_type: "Temperature".to_string(),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: "Kitchen_ThermostatTemperature".to_string(),
        source: "Kitchen".to_string(),
        sensor_type: "ThermostatTemperature".to_string(),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: "Kitchen_Virtual_OutdoorTemperature".to_string(),
        source: "Kitchen".to_string(),
        sensor_type: "VirtualOutdoorTemperature".to_string(),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
    Sensor {
        name: "Kitchen_Humidity".to_string(),
        source: "Kitchen".to_string(),
        sensor_type: "Humidity".to_string(),
        sampling_rate: GLOBAL_SAMPLING_RATE,
    },
];

impl Sensor {
    pub fn sense(
        self: &Self,
        gateway_url: impl AsRef<str>,
    ) {
        Logger::info(
            &format!("Sending data from sensor {}...", &self.name),
            true,
        );

        let dataset = SensorDataset::new(&self.name);

        for value in dataset.iter() {
            let datapoint = SensorDatapoint {
                source: self.source.clone(),
                sensor_type: self.sensor_type.clone(),
                timestamp: chrono::Utc::now().timestamp(),
                value: value.unwrap(),
            };
            Logger::info(
                &format!(
                    "Sensed data from sensor {}: {} at {}",
                    self.name, datapoint.value, datapoint.timestamp
                ),
                true,
            );
            // Send data to the gateway via websocket.
        }
    }
}
