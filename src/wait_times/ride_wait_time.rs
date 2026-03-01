use crate::wait_times::{queue_times::QueueTimesRide, utils::get_wait_time_color, WaitTimeColor};

pub struct RideWaitTime {
    name: String,
    wait_time: String,
    color: WaitTimeColor
}

impl RideWaitTime {
    fn from_queue_time(queue_time_data: QueueTimesRide) -> Self {
        let wait_time: String;
        let color: WaitTimeColor;

        // Check if ride is open
        if !queue_time_data.is_open {
            // Ride is closed
            wait_time = String::from("Closed");
            color = WaitTimeColor::Red;
        } else {
            // Ride is open
            wait_time = queue_time_data.wait_time.to_string();
            color = match get_wait_time_color(&queue_time_data.name, queue_time_data.wait_time) {
                Some(color) => color,
                None => {
                    WaitTimeColor::Green
                }
            };
        }

        RideWaitTime {
            name: queue_time_data.name,
            wait_time,
            color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_queue_time {
        use super::*;

        #[test]
        fn it_returns_correct_struct_with_valid_open_ride_data() {
            let result = RideWaitTime::from_queue_time(QueueTimesRide {
                id: 100,
                name: String::from("Corkscrew"),
                is_open: true,
                wait_time: 10,
                last_updated: String::new(),
            });

            assert_eq!(result.name, "Corkscrew");
            assert_eq!(result.wait_time, "10");
            assert_eq!(result.color, WaitTimeColor::Green);
        }

        #[test]
        fn it_returns_correct_struct_with_valid_closed_ride_data() {
            let result = RideWaitTime::from_queue_time(QueueTimesRide {
                id: 100,
                name: String::from("Corkscrew"),
                is_open: false,
                wait_time: 10,
                last_updated: String::new()
            });

            assert_eq!(result.name, "Corkscrew");
            assert_eq!(result.wait_time, "Closed");
            assert_eq!(result.color, WaitTimeColor::Red);
        }
    }
}
