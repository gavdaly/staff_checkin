/// Filters a phone number string by removing non-numeric characters and the leading '1'.
///
/// # Arguments
///
/// * `number` - A string representing the phone number to be filtered.
///
/// # Example
///
/// ```
/// # use staff::utils::filter_phone_number;
/// let number = "+1 (800) 222-3333";
/// let filtered_number = filter_phone_number(number);
/// println!("{}", filtered_number);
/// // Output: "8002223333"
/// ```
pub fn filter_phone_number(number: &str) -> String {
    let (_, n) = number.chars().fold(
        (false, String::with_capacity(10)),
        |(first, mut string), c| {
            if !c.is_numeric() {
                return (first, string);
            };
            if !first && c == '1' {
                return (true, string);
            }
            string.push(c);
            (true, string)
        },
    );
    n
}

/// Formats a phone number string into a formatted phone number string.
///
/// # Arguments
///
/// * `number` - A string representing a phone number.
///
/// # Example
///
/// ```
/// # use staff::utils::format_phone_number;
/// let number = "2345677890";
/// let formatted_number = format_phone_number(number);
/// assert_eq!(formatted_number, "+1 (234) 567-7890")
/// ```
pub fn format_phone_number(number: &str) -> String {
    let (area, number) = number.split_at(3);
    let (middle, last) = number.split_at(3);
    format!("+1 ({area}) {middle}-{last}")
}

/// Calculates the distance between two points on the Earth's surface using the Haversine formula.
///
/// # Arguments
///
/// * `lat1` - Latitude of the first point in degrees.
/// * `lon1` - Longitude of the first point in degrees.
/// * `lat2` - Latitude of the second point in degrees.
/// * `lon2` - Longitude of the second point in degrees.
///
/// # Example
///
/// ```
/// let lat1 = 37.7749;
/// let lon1 = -122.4194;
/// let lat2 = 34.0522;
/// let lon2 = -118.2437;
/// // let distance = calculate_distance(lat1, lon1, lat2, lon2);
/// // assert_eq!(distance, 559.2);
/// ```
pub fn caluclate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    use std::f64::consts::PI;
    let r = 6371000.; // metres
    let o1 = lat1 * PI / 180.; // φ, λ in radians
    let o2 = lat2 * PI / 180.;
    let d_o = (lat2 - lat1) * PI / 180.;
    let dl = (lon2 - lon1) * PI / 180.;

    let a = (d_o / 2.).sin() * (d_o / 2.).sin()
        + o1.cos() * o2.cos() * (dl / 2.).sin() * (dl / 2.).sin();
    let c = 2. * a.sqrt().atan2((1. - a).sqrt());

    r * c
}

/// Converts a duration in milliseconds to hours.
///
/// # Arguments
///
/// * `duration` - A reference to an `i64` duration in milliseconds.
///
/// # Example
///
/// ```
/// # use staff::utils::miliseconds_to_hour;
/// let duration = 3661000;
/// let result = miliseconds_to_hour(&duration);
/// println!("{}", result);
/// // Output: 1.0175
/// ```
pub fn miliseconds_to_hour(duration: &i64) -> f64 {
    *duration as f64 / 1000. / 60. / 60.
}

/// Converts a duration in milliseconds to hours, minutes, and seconds.
///
/// # Arguments
///
/// * `duration` - A reference to an `i64` duration in milliseconds.
///
/// # Example
///
/// ```
/// # use staff::utils::miliseconds_to_hour_minute;
/// let duration = 3661000;
/// let result = miliseconds_to_hour_minute(&duration);
/// println!("{:?}", result);
/// // Output: (1, 1, 1)
/// ```
pub fn miliseconds_to_hour_minute(duration: &i64) -> (i64, i64, i64) {
    let hours = duration / 1000 / 60 / 60;
    let minutes = duration / 1000 / 60 % 60;
    let seconds = duration / 1000 % 60;
    (hours, minutes, seconds)
}

/// Converts a duration in milliseconds to a formatted string representation.
///
/// # Arguments
///
/// * `duration` - A reference to the duration in milliseconds.
///
/// # Returns
///
/// * `#.##h (#h ##m)`
/// and when there are no hours:
/// * `#.####h (#m #s)`
///
/// # Example
///
/// ```
/// # use staff::utils::miliseconds_to_string;
/// let duration = 3661000;
/// let result = miliseconds_to_string(&duration);
/// println!("{}", result);
/// // Output: "1.02h (1h 1m)"
/// ```
pub fn miliseconds_to_string(duration: &i64) -> String {
    let hours_dec = miliseconds_to_hour(duration);
    let (hours, minutes, seconds) = miliseconds_to_hour_minute(duration);
    match hours {
        0 => format!("{:.4}h ({}m {}s)", hours_dec, minutes, seconds),
        h => format!("{:.2}h ({}h {}m)", hours_dec, h, minutes),
    }
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn test_format_phone_number() {
        assert_eq!(
            format_phone_number("2345677890"),
            "+1 (234) 567-7890".to_string()
        )
    }

    #[test]
    fn test_miliseconds_to_string() {
        let a = (60 * 60 * 1000) - 500;
        assert_eq!(miliseconds_to_string(&a), "0.9999h (59m 59s)".to_string());
    }

    #[test]
    fn tests_filtering_a_phone_number() {
        assert_eq!(filter_phone_number("+1 (800) 222-3333"), "8002223333");
        assert_eq!(filter_phone_number("18002223333"), "8002223333");
        assert_eq!(filter_phone_number("1(800)222-3333"), "8002223333");
    }
}
