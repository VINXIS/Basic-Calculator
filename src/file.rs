use std::{fs, path::PathBuf, os::windows::fs::MetadataExt};

enum FileIcon {
    Directory,
    File,
    Symlink,
    Unknown
}

fn get_file_type(file_type: &fs::FileType) -> FileIcon {
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

fn get_file_icon(file_type: &fs::FileType) -> String {
    match get_file_type(file_type) {
        FileIcon::Directory => String::from("📁"),
        FileIcon::File => String::from("📄"),
        FileIcon::Symlink => String::from("🔗"),
        FileIcon::Unknown => String::from("❓")
    }
}

fn append_level (level: u8) -> String {
    let output = String::from(" ").repeat((level - 1) as usize);
    if level > 1 {
        format!("{}└", output)
    } else {
        String::from("")
    }
}

pub fn output_folder(path: &PathBuf, levels: u8, current_level: u8, hidden: &bool) {
    let paths = match std::fs::read_dir(path.clone()) {
        Ok(paths) => paths,
        Err(err) => {
            println!("Could not read directory \"{}\" due to the error:\n{}", path.display(), err);
            return
        }
    };

    for path in paths {
        let path = path.unwrap();
        let file_type = path.file_type().unwrap();
        let file_name = path.file_name().into_string().unwrap();
        if  (
            (!cfg!(windows) && file_name.starts_with(".")) ||
            (cfg!(windows) && path.metadata().unwrap().file_attributes() & 2 == 2)
        ) && 
        !hidden {
            continue;
        }

        println!("{}{} {}", append_level(current_level), get_file_icon(&file_type), file_name);
        if file_type.is_dir() && current_level < levels {
            output_folder(&path.path(), levels, current_level + 1, hidden);
        }
    }
}