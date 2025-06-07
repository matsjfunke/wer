pub fn format_timestamp_day_month(timestamp: i64) -> String {
    // Convert Unix timestamp to short format like "22 May" or "07 Jun" (no year)
    let datetime =
        chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| chrono::Utc::now());

    datetime.format("%d %b").to_string()
}

pub fn format_timestamp_day_month_year(timestamp: i64) -> String {
    // Convert Unix timestamp to format like "22 May 2025" or "07 Jun 2025" (with year)
    let datetime =
        chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| chrono::Utc::now());

    datetime.format("%d %b %Y").to_string()
} 
