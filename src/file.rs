use std::{fs::{FileType, read_dir, Metadata}, path::PathBuf};

use crate::{util::{unicode_support, parse_file_size, hidden_check}, Sort, error::{FileTimeError, handle_io_error, handle_file_time_error}};

enum FileIcon {
    Directory,
    File,
    Symlink,
    Unknown
}

pub enum GetFile {
    File(FileInfo),
    Size(u64)
}

pub struct FileInfo {
    file_level: u8,
    file_name: String,
    file_type: FileType,
    created: u64,
    pub file_size: u64,
    pub sub_files: Option<Vec<GetFile>>
}

fn get_file_type(file_type: &FileType) -> FileIcon {
    match file_type {
        _ if file_type.is_dir() => FileIcon::Directory,
        _ if file_type.is_symlink() => FileIcon::Symlink,
        _ if file_type.is_file() => FileIcon::File,
        _ => FileIcon::Unknown,
    }
}

fn get_file_icon(file_type: &FileType) -> String {
    if unicode_support() {
        match get_file_type(file_type) {
            FileIcon::Directory => String::from("📁"),
            FileIcon::File => String::from("📄"),
            FileIcon::Symlink => String::from("🔗"),
            FileIcon::Unknown => String::from("❓")
        }
    } else {
        match get_file_type(file_type) {
            FileIcon::Directory => String::from("[D]"),
            FileIcon::File => String::from("[F]"),
            FileIcon::Symlink => String::from("[S]"),
            FileIcon::Unknown => String::from("[?]")
        }
    }
}

fn get_created_since(metadata: &Metadata) -> Result<u64, FileTimeError> {
    let creation_time = metadata.created()?;
    let elapsed = creation_time.elapsed()?;
    Ok(elapsed.as_secs())
}

fn get_file_metadata_and_created_since(path: &PathBuf) -> Result<(Metadata, u64), FileTimeError> {
    let metadata = path.metadata()?;
    let created = get_created_since(&metadata)?;
    Ok((metadata, created))
}

fn append_level(level: &u8) -> String {
    if level > &1 {
        format!("{}└", String::from(" ").repeat((level - 1) as usize))
    } else {
        String::from("")
    }
}

// For deeper levels to get file size
fn get_files_oos(path: &PathBuf, hidden: &bool, verbose: &bool) -> u64 {
    let dir = match read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            handle_io_error(&path, err);
            return 0;
        }
    };

    if *verbose {
        println!("Reading {}...", path.display());
    }

    let mut size: u64 = 0;
    for entry in dir.filter_map(Result::ok) {
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                handle_io_error(&entry.path(), err);
                continue;
            }
        };
        

        #[cfg(target_os = "windows")]
        if hidden_check(&metadata, hidden) {
            continue;
        }

        #[cfg(target_os = "linux")]
        {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if hidden_check(&file_name, hidden) {
                continue;
            }
        }

        if metadata.is_dir() {
            size += get_files_oos(&entry.path(), hidden, verbose);
        } else {
            size += metadata.len();
        }
    }

    size
}

pub fn get_files(
    path: &PathBuf,
    current_level: u8,
    levels: &u8,
    hidden: &bool,
    verbose: &bool
) -> GetFile {
    let dir = match read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            handle_io_error(&path, err);
            return GetFile::Size(0);
        }
    };
    let (metadata, created) = match get_file_metadata_and_created_since(path) {
        Ok((metadata, created)) => (metadata, created),
        Err(err) => {
            handle_file_time_error(&path, err);
            return GetFile::Size(0);
        }
    };

    if *verbose {
        println!("Reading {}...", path.display());
    }

    let mut sub_files: Vec<GetFile> = Vec::new();
    for entry in dir.filter_map(Result::ok) {
        let (metadata, created) = match get_file_metadata_and_created_since(&entry.path()) {
            Ok((metadata, created)) => (metadata, created),
            Err(err) => {
                handle_file_time_error(&entry.path(), err);
                continue;
            }
        };

        let file_name = entry.file_name().into_string().unwrap_or_default();

        #[cfg(target_os = "windows")]
        if hidden_check(&metadata, hidden) {
            continue;
        }

        #[cfg(target_os = "linux")]
        if hidden_check(&file_name, hidden) {
            continue;
        }

        if metadata.is_dir() {
            if current_level >= *levels {
                sub_files.push(GetFile::File(FileInfo {
                    file_level: current_level,
                    file_name,
                    file_type: metadata.file_type(),
                    created: created,
                    file_size: metadata.len() + get_files_oos(&entry.path(), hidden, verbose),
                    sub_files: None
                }));
            } else {
                sub_files.push(get_files(&entry.path(), current_level + 1, levels, hidden, verbose));
            }
        } else {
            sub_files.push(GetFile::File(FileInfo {
                file_level: current_level,
                file_name,
                file_type: metadata.file_type(),
                created: created,
                file_size: metadata.len(),
                sub_files: None
            }));
        }
    }

    GetFile::File(FileInfo {
        file_level: current_level - 1,
        file_name: path.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string(),
        file_type: metadata.file_type(),
        created: created,
        file_size: sub_files.iter().map(|file| match file {
            GetFile::File(file) => file.file_size,
            GetFile::Size(size) => *size
        }).sum::<u64>(),
        sub_files: Some(sub_files)
    })
}

fn print_file(file: &FileInfo) -> String {
    format!(
        "{}{} {} ({})", 
        append_level(&file.file_level), 
        get_file_icon(&file.file_type), 
        file.file_name, 
        parse_file_size(file.file_size)
    )
}

pub fn print_sub_files(sub_files: &[GetFile], sort: &Sort) {
    let mut files = sub_files.iter().filter_map(|file| match file {
        GetFile::File(file) => Some(file),
        GetFile::Size(_) => None
    }).collect::<Vec<&FileInfo>>();
    files.sort_by(|f1, f2| match sort {
            Sort::NameASC => f1.file_name.cmp(&f2.file_name),
            Sort::NameDEC => f2.file_name.cmp(&f1.file_name),
            Sort::SizeASC => f1.file_size.cmp(&f2.file_size),
            Sort::SizeDEC => f2.file_size.cmp(&f1.file_size),
            Sort::DateASC => f1.created.cmp(&f2.created),
            Sort::DateDEC => f2.created.cmp(&f1.created)
    });
    for file in files {
        println!("{}", print_file(file));

        let sub_files = match &file.sub_files {
            Some(sub_files) => sub_files,
            None => continue
        };

        print_sub_files(sub_files, sort);
    }
}