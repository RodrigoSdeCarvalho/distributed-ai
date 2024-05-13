use polars::prelude::*;
use std::fmt::Display;

use system::{Path, Logger};

#[derive(Debug, Clone)]
pub(crate) struct SensorDataset {
    df: DataFrame,
}

impl SensorDataset {
    pub fn new<T>(sensor_name: T) -> Self
    where
        T: AsRef<str> + Display,
    {
        let path = Path::get_data(
            sensor_name.to_string() + ".csv"
        );
        let mut df = CsvReader::from_path(path)
            .unwrap_or_else(|e| {
                Logger::error(
                    &format!("Failed to read sensor data file: {}", e),
                    true
                );
                panic!("Error reading CSV file");
            })
            .infer_schema(None)
            .has_header(false)
            .finish()
            .unwrap();

        df.rename("column_1", "timestamp")
            .unwrap();
        df.rename("column_2", "value")
            .unwrap();

        // Timestamp will be created on simulation
        df.drop("timestamp")
            .unwrap();

        let values = df.column("value")
            .unwrap()
            .cast(&DataType::Float32)
            .unwrap();
        df.with_column(values)
            .unwrap();

        Self { df }
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<f32>> + '_ {
        self.df
            .column("value")
            .unwrap()
            .f32()
            .unwrap()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Error reading CSV file")]
    fn test_nonexistent_sensor_data() {
        let _ = SensorDataset::new("test");
    }

    #[test]
    fn test_sensor_data() {
        let sensor_data = SensorDataset::new("Kitchen_Brightness");
        println!("{:?}", sensor_data);
    }

    #[test]
    fn test_sensor_data_iter() {
        let sensor_data = SensorDataset::new("Kitchen_Brightness");
        for value in sensor_data.iter() {
            println!("{}", value.unwrap());
        }
    }
}
