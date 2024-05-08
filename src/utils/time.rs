use chrono::{DateTime, FixedOffset, Local};

pub fn now() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}

pub fn from_string(time: &String) -> DateTime<FixedOffset> {
    time.parse::<DateTime<FixedOffset>>().expect(&format!("{} should be parsable as time string", time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let parsed = from_string(&String::from("2023-12-31T12:34:56+01:00"));
        assert_eq!(1704022496, parsed.timestamp());
    }
    #[test]
    #[should_panic]
    fn test_from_string_fails_on_invalid_input() {
        let _ = from_string(&String::from("this is not a time string"));
        // code should have paniced here
    }
}