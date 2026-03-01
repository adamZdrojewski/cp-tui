use crate::wait_times::WaitTimeColor;

pub fn get_wait_time_color(ride_name: &str, wait_time: u32) -> Option<WaitTimeColor> {
    match ride_name {
        "Blue Streak" => {
            return Some(over_under(12, 25, wait_time));
        },
        "Cedar Creek Mine Ride" => {
            return Some(over_under(10, 22, wait_time));
        },
        "Corkscrew" => {
            return Some(over_under(13, 25, wait_time));
        },
        "GateKeeper" => {
            return Some(over_under(19, 41, wait_time));
        },
        "Gemini" => {
            return Some(over_under(18, 38, wait_time));
        },
        "Iron Dragon" => {
            return Some(over_under(13, 26, wait_time));
        },
        "Magnum XL-200" => {
            return Some(over_under(17, 46, wait_time));
        },
        "Maverick" => {
            return Some(over_under(31, 69, wait_time));
        },
        "Millennium Force" => {
            return Some(over_under(27, 63, wait_time));
        },
        "Raptor" => {
            return Some(over_under(16, 43, wait_time));
        },
        "Rougarou" => {
            return Some(over_under(11, 25, wait_time));
        },
        "Siren's Curse" => {
            return Some(over_under(30, 100, wait_time));
        },
        "Steel Vengeance" => {
            return Some(over_under(27, 72, wait_time));
        },
        "Top Thrill 2" => {
            return Some(over_under(45, 100, wait_time));
        },
        "Valravn" => {
            return Some(over_under(27, 37, wait_time));
        },
        "Wild Mouse" => {
            return Some(over_under(10, 20, wait_time));
        },
        _ => {
            return None;
        }
    }
}

pub fn over_under(low: u32, high: u32, wait_time: u32) -> WaitTimeColor {
    if wait_time <= low {
        return WaitTimeColor::Green;
    } else if wait_time >= high {
        return WaitTimeColor::Red;
    } else {
        return WaitTimeColor::Yellow;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_wait_time_color {
        use super::*;

        #[test]
        fn it_returns_color_with_valid_ride() {
            let result = get_wait_time_color("Corkscrew", 10);
            assert_eq!(result, Some(WaitTimeColor::Green));
        }

        #[test]
        fn it_returns_none_with_invalid_ride() {
            let result = get_wait_time_color("Top Thrill Dragster", 100);
            assert_eq!(result, None);
        }
    }

    mod over_under {
        use super::*;

        #[test]
        fn it_returns_green() {
            let result = over_under(10, 20, 10);
            assert_eq!(result, WaitTimeColor::Green);
        }

        #[test]
        fn it_returns_yellow() {
            let result = over_under(10, 20, 15);
            assert_eq!(result, WaitTimeColor::Yellow);
        }
        #[test]
        fn it_returns_red() {
            let result = over_under(10, 20, 20);
            assert_eq!(result, WaitTimeColor::Red);
        }
    }
}
