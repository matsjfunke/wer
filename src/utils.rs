use chrono::{DateTime, Utc};

pub fn format_timestamp_day_month(timestamp: i64) -> String {
    // Convert Unix timestamp to short format like "22 May" or "07 Jun" (no year)
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now);

    dt.format("%d %b").to_string()
}

pub fn format_timestamp_day_month_year(timestamp: i64) -> String {
    // Convert Unix timestamp to format like "22 May 2025" or "07 Jun 2025" (with year)
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now);

    dt.format("%d %b %Y").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_day_month() {
        let timestamp = 1749456964;
        let result = format_timestamp_day_month(timestamp);
        assert_eq!(result, "09 Jun");
    }

    #[test]
    fn test_format_timestamp_day_month_year() {
        let timestamp = 1749456964;
        let result = format_timestamp_day_month_year(timestamp);
        assert_eq!(result, "09 Jun 2025");
    }
}

