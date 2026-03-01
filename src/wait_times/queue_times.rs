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
pub struct QueueTimesRide {
    pub id: u32,
    pub name: String,
    pub is_open: bool,
    pub wait_time: u32,
    pub last_updated: String
}

async fn fetch_wait_time_data() -> Result<QueueTimesResponse, reqwest::Error> {
    #[cfg(not(feature = "mock"))]
    {
        let url = "https://queue-times.com/parks/50/queue_times.json";
        let response = reqwest::get(url).await?;
        let wait_time_data: QueueTimesResponse = response.json().await?;
        Ok(wait_time_data)
    }

    #[cfg(feature = "mock")]
    {
        println!("Loading mock data from file...");
        let file_data = std::fs::read_to_string("mock-data.json").unwrap();
        let data = serde_json::from_str(&file_data).unwrap();
        Ok(data)
    }
}
