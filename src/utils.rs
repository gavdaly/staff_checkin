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
