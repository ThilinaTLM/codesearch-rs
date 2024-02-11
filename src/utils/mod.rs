use std::path::Path;

pub fn get_language(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "java" => Some("java".to_string()),
        "kt" => Some("kotlin".to_string()),
        "rs" => Some("rust".to_string()),
        "go" => Some("go".to_string()),
        "py" => Some("python".to_string()),
        "js" => Some("javascript".to_string()),
        "ts" => Some("typescript".to_string()),
        "html" => Some("html".to_string()),
        "css" => Some("css".to_string()),
        "json" => Some("json".to_string()),
        "xml" => Some("xml".to_string()),
        "yaml" => Some("yaml".to_string()),
        "yml" => Some("yaml".to_string()),
        "toml" => Some("toml".to_string()),
        "md" => Some("markdown".to_string()),
        "sh" => Some("shell".to_string()),
        "bat" => Some("batch".to_string()),
        "ps1" => Some("powershell".to_string()),
        "c" => Some("c".to_string()),
        "h" => Some("c".to_string()),
        "cpp" => Some("cpp".to_string()),
        "hpp" => Some("cpp.h".to_string()),
        _ => Some(ext.to_string()),
    }
}

pub fn convert_datetime_chrono_to_tantivy(dt: &chrono::DateTime<chrono::Utc>) -> tantivy::DateTime {
    tantivy::DateTime::from_timestamp_millis(dt.timestamp_millis())
}