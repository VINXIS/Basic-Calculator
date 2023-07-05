use std::{path::PathBuf, io::{Error, ErrorKind}, time::SystemTimeError};

pub enum FileTimeError {
    IoError(Error),
    ElapsedError(SystemTimeError),
}

impl From<Error> for FileTimeError {
    fn from(err: Error) -> FileTimeError {
        FileTimeError::IoError(err)
    }
}

impl From<SystemTimeError> for FileTimeError {
    fn from(err: SystemTimeError) -> FileTimeError {
        FileTimeError::ElapsedError(err)
    }
}

pub fn handle_file_time_error(path_buf: &PathBuf, err: FileTimeError) {
    match err {
        FileTimeError::IoError(err) => handle_io_error(path_buf, err),
        FileTimeError::ElapsedError(err) => println!("Could not get elapsed time for file \"{}\" due to the error:\n{}", path_buf.display(), err)
    }
}

pub fn handle_io_error(path_buf: &PathBuf, err: Error) {
    match err.kind() {
        ErrorKind::NotFound => println!("The specified path {} probably doesn't exist", path_buf.display()),
        ErrorKind::PermissionDenied => println!("The program doesn't have permission to access {}", path_buf.display()),
        _ => println!("Could not read directory \"{}\" due to the error:\n{}", path_buf.display(), err)
    }
}