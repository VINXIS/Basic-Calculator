use std::{io::{Error, ErrorKind}, fs::{FileType, read_dir, metadata, DirEntry}, path::PathBuf, os::windows::fs::MetadataExt};

use crate::util::{unicode_support, parse_file_size};

enum FileIcon {
    Directory,
    File,
    Symlink,
    Unknown
}

fn get_file_type(file_type: &FileType) -> FileIcon {
    if file_type.is_dir() {
        FileIcon::Directory
    } else if file_type.is_file() {
        FileIcon::File
    } else if file_type.is_symlink() {
        FileIcon::Symlink
    } else {
        FileIcon::Unknown
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

fn calculate_dir_size(
    main_path: &PathBuf,
    hidden: &bool
) -> u64 {
    let paths = match read_dir(main_path) {
        Ok(paths) => paths,
        Err(err) => {
            println!("Could not read directory \"{}\" due to the error:\n{}", main_path.display(), err);
            return 0
        }

    };
    let mut size: u64 = 0;

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(err) => {
                println!("Could not read sub_path in directory \"{}\" due to the error:\n{}", main_path.display(), err);
                continue
            }
        };
        match (|| -> Result<(), Error> {
            let metadata = metadata(&path.path())?;
            let file_type = path.file_type()?;

            if file_type.is_dir() {
                size += metadata.len() + calculate_dir_size(&path.path(), hidden);
            } else {
                size += metadata.len();
            }
            Ok(())
        })() {
            Ok(_) => {},
            Err(err) => println!("Could not read directory \"{}\" due to the error:\n{}", path.path().display(), err)
        }
    }

    size
}

fn append_level (level: u8) -> String {
    if level > 1 {
        format!("{}└", String::from(" ").repeat((level - 1) as usize))
    } else {
        String::from("")
    }
}

fn print_file (current_level: u8, file_type: &FileType, file_name: &str, file_size: u64) {
    println!(
        "{}{} {} ({})", 
        append_level(current_level), 
        get_file_icon(file_type), 
        file_name, 
        parse_file_size(file_size)
    );
}

fn output_sub_path(
    path: &DirEntry, 
    levels: u8, 
    current_level: u8, 
    hidden: &bool, 
    counter: &mut u32,
    size_count: &mut u64
) -> Result<(), Error> {
    let file_name = path.file_name().into_string().map_err(|_| Error::new(ErrorKind::Other, "Invalid filename"))?;
    let metadata = metadata(&path.path())?;

    if (
        (!cfg!(windows) && file_name.starts_with(".")) ||
        (cfg!(windows) && metadata.file_attributes() & 2 == 2)
    ) && 
    !hidden {
        return Ok(());
    }
    
    let file_type = path.file_type()?;

    if current_level <= levels {
        if file_type.is_dir() {
            let dir_size = metadata.len() + calculate_dir_size(&path.path(), hidden);

            print_file(current_level, &file_type, &file_name, dir_size);
            output_path(&path.path(), levels, current_level + 1, hidden, counter, size_count);
            
            if current_level == 1 {
                *size_count += dir_size;
            }
        } else {
            print_file(current_level, &file_type, &file_name, metadata.len());
        }
    }

    *counter += 1;
    return Ok(());
}

pub fn output_path(
    main_path: &PathBuf, 
    levels: u8, 
    current_level: u8, 
    hidden: &bool, 
    counter: &mut u32,
    size_count: &mut u64
) {
    let paths = match read_dir(main_path.clone()) {
        Ok(paths) => paths,
        Err(err) => {
            println!("Could not read directory \"{}\" due to the error:\n{}", main_path.display(), err);
            return
        }
    };

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(err) => {
                println!("Could not read directory \"{}\" due to the error:\n{}", main_path.display(), err);
                continue
            }
        };
        if let Err(err) = output_sub_path(&path, levels, current_level, hidden, counter , size_count) {
            println!("Could not read directory \"{}\" due to the error:\n{}", path.path().display(), err)
        }
    }
}