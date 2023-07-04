use std::env::var;

pub fn unicode_support() -> bool {
    var("LANG").unwrap_or_default().to_lowercase().contains("utf-8")
}

pub fn parse_file_size (file_size: u64) -> String {
    let mut file_size = file_size as f64;
    let file_size_units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut unit = 0;

    while file_size > 1024.0 && unit < file_size_units.len() - 1 {
        file_size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", file_size, file_size_units[unit])
}