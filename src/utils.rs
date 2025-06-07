pub fn format_timestamp(timestamp: i64) -> String {
    // Convert Unix timestamp to YYYY-MM-DD format
    let datetime =
        chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| chrono::Utc::now());

    datetime.format("%Y-%m-%d").to_string()
}

pub fn format_timestamp_day_month(timestamp: i64) -> String {
    // Convert Unix timestamp to short format like "22 May" or "07 Jun"
    let datetime =
        chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| chrono::Utc::now());

    datetime.format("%d %b").to_string()
} 
