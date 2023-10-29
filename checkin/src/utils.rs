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

pub fn format_phone_number(number: &str) -> String {
    let (area, number) = number.split_at(3);
    let (middle, last) = number.split_at(3);
    format!("+1 ({area}) {middle}-{last}")
}

/// calculates the distance between to points in meters
pub fn caluclate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    use std::f64::consts::PI;
    let r = 6371000.; // metres
    let o1 = lat1 * PI / 180.; // φ, λ in radians
    let o2 = lat2 * PI / 180.;
    let d_o = (lat2 - lat1) * PI / 180.;
    let dl = (lon2 - lon1) * PI / 180.;

    let a =
        (d_o / 2.).sin() * (d_o / 2.).sin() + o1.cos() * o2.cos() * (dl / 2.).sin() * (dl / 2.).sin();
    let c = 2. * a.sqrt().atan2((1. - a).sqrt());

    r * c
}

#[cfg(test)]
mod utils_test {
    use crate::utils::format_phone_number;

    #[test]
    fn test_format_phone_number() {
        assert_eq!(
            format_phone_number("2345677890"),
            "+1 (234) 567-7890".to_string()
        )
    }
}
