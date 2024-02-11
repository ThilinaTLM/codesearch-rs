pub fn convert_datetime_chrono_to_tantivy(dt: &chrono::DateTime<chrono::Utc>) -> tantivy::DateTime {
    tantivy::DateTime::from_timestamp_millis(dt.timestamp_millis())
}