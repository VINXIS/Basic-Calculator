use std::{env::var, fs::Metadata, os::windows::fs::MetadataExt};

pub enum Sort {
    NameASC,
    NameDEC,
    SizeASC,
    SizeDEC,
    DateASC,
    DateDEC
}

pub fn unicode_support() -> bool {
    var("LANG").unwrap_or_default().to_lowercase().contains("utf-8")
}

pub fn parse_file_size(file_size: u64) -> String {
    let file_size_units: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut file_size = file_size as f64;
    let mut unit = 0;

    while file_size > 1024.0 && unit < file_size_units.len() - 1 {
        file_size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", file_size, file_size_units[unit])
}

pub fn hidden_check (metadata: &Metadata, file_name: &String, hidden: &bool) -> bool {
    (
        (!cfg!(windows) && file_name.starts_with(".")) ||
        (cfg!(windows) && metadata.file_attributes() & 2 == 2)
    ) && !hidden
}