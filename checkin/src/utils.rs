/// Filters out the leading 1 from a North American phone number and only keep the remaining digits
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

/// Formats a phonenumber from an North Americian 10 digit number
pub fn format_phone_number(number: &str) -> String {
    let (area, number) = number.split_at(3);
    let (middle, last) = number.split_at(3);
    format!("+1 ({area}) {middle}-{last}")
}

/// Calculates the distance between to points in meters
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

/// Convert miliseconds to decimal hours
pub fn miliseconds_to_hour(duration: &i64) -> f64 {
    *duration as f64 / 1000. / 60. / 60.
}

/// Convert miliseconds to hours and minutes and seconds
pub fn miliseconds_to_hour_minute(duration: &i64) -> (i64, i64, i64) {
    let hours = duration / 1000 / 60 / 60;
    let minutes = duration / 1000 / 60 % 60;
    let seconds = duration / 1000 % 60;
    (hours, minutes, seconds)
}

/// Convert miliseconds to a string:
/// `#.##h (#h ##m)`
/// and when there are no hours:
/// `#.#### (#m #s)`
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
        let a = (1 * 60 * 60 * 1000) - 1;
        assert_eq!(miliseconds_to_string(a), "0.9999h (59m 59s)".to_string());
    }

    #[test]
    fn tests_filtering_a_phone_number() {
        assert_eq!(filter_phone_number("+1 (800) 222-3333"), "8002223333");
        assert_eq!(filter_phone_number("18002223333"), "8002223333");
        assert_eq!(filter_phone_number("1(800)222-3333"), "8002223333");
    }
}
