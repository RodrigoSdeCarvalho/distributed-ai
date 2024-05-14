#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use sensors::sensor::SensorDatapoint;

#[derive(Serialize)]
struct ComposeSensorData {
    items: Vec<SensorDatapoint>
}

#[derive(Serialize)]
struct StatusMessage {
    message: String
}


#[post("/push_data", format = "json", data = "<item>")]
fn push_data(item: Json<SensorDatapoint>) -> Result<Json<StatusMessage>, String> {
    // Extract the SensorDataPoint from the Json wrapper
    let sensor_data_point: SensorDatapoint = item.into_inner();

    // Assuming you have a storage mechanism to store/retrieve SensorDataPoints
    // Here you can add the logic to store the received SensorDataPoint in a database or other storage

    // Example: Store the extracted SensorDataPoint in a database or vector
    let mut data_store: Vec<SensorDatapoint> = vec![];
    data_store.push(sensor_data_point); // Store the received SensorDataPoint

    // Check if there are enough items to create a batch of five
    if data_store.len() >= 5 {
        let mut batches: Vec<ComposeSensorData> = vec![];
        let mut current_batch: Vec<SensorDatapoint> = vec![];

        batches.push(ComposeSensorData { items: current_batch, });


        // Clear the data_store after processing
    data_store.clear();

    Ok(Json(StatusMessage {
        message: format!("Received and processed data successfully. Created {} batches.", batches.len()),
    }))
    }
    else
    { Ok(Json(StatusMessage {
            message: format!("Received data successfully. Waiting for more data to create a batch."),
        }))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, push_data])
}
