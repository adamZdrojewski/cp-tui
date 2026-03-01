use crate::wait_times::ride_wait_time::RideWaitTime;

mod queue_times;
mod ride_wait_time;
mod utils;

#[derive(Debug, PartialEq)]
enum WaitTimeColor {
    Green,
    Yellow,
    Red
}

pub fn get_wait_times() -> Vec<RideWaitTime> {
    vec![]
}
