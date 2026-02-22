use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueueTimesResponse {
    lands: Vec<QueueTimesLand>
}

#[derive(Deserialize, Debug)]
struct QueueTimesLand {
    id: u32,
    name: String,
    rides: Vec<QueueTimesRide>
}

#[derive(Deserialize, Debug)]
struct QueueTimesRide {
    id: u32,
    name: String,
    is_open: bool,
    wait_time: u32,
    last_updated: String
}

async fn fetch_wait_time_data() -> Result<QueueTimesResponse, reqwest::Error> {
    let url = "https://queue-times.com/parks/50/queue_times.json";

    let response = reqwest::get(url).await?;

    let wait_time_data: QueueTimesResponse = response.json().await?;

    Ok(wait_time_data)
}
