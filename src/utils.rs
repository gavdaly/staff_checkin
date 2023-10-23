use cfg_if::cfg_if;

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

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::{Connection, PgConnection};
    use leptos::ServerFnError;

    pub async fn db() -> Result<PgConnection, ServerFnError> {
        match PgConnection::connect("").await {
            Ok(connection) => Ok(connection),
            Err(err) => Err(ServerFnError::ServerError("Error Connecting DB".to_string()))
        }
    }
}
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
