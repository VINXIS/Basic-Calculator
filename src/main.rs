use std::{env::{args, current_dir}, process::exit, path::Path, time::Instant};

use crate::{file::{get_files, GetFile, print_sub_files}, util::{unicode_support, parse_file_size, Sort}};

mod error;
mod file;
mod util;

fn print_help() {
    println!("Usage: treedir [OPTIONS]
    -d, --dir <PATH>                The directory to list (Default: current directory)
    -l, --levels <NUM>              The number of levels to list (Default: 1)
    -s, --sort <TYPE> [ASC/DEC]     The type of sort to use (Default: name ASC)
                                    Valid types: name, size, date
                                    Optional: ASC (ascending), DEC (descending)
    -n, --hidden                    Show hidden files and folders
    -v, --verbose                   Show verbose output (which folders are currently being read)
    -h, --help                      Prints help information");
}

fn main() {
    let start = Instant::now();
    let args = args().collect::<Vec<String>>();
    let hidden = args.contains(&String::from("--hidden")) || args.contains(&String::from("-n"));
    let verbose: bool = args.contains(&String::from("--verbose")) || args.contains(&String::from("-v"));
    let mut path = current_dir().unwrap_or_default();
    let mut levels: u8 = 1;
    let mut sort: Sort = Sort::NameASC;
    for i in 0..args.len() {
        let arg = &args[i];
        if  arg == "--help" || arg == "-h" {
            print_help();
            exit(0);
        }

        if (arg == "--dir" || arg == "-d") && args.len() > i + 1 {
            // Get the next arg and validate the path
            let next_arg = &args[i+1];
            let next_path = Path::new(next_arg);
            if next_path.is_dir() {
                path = next_path.to_path_buf();
            } else {
                panic!("{} is not a valid path", next_arg);
            }
            continue;
        }

        if (arg == "--levels" || arg =="-l") && args.len() > i + 1 {
            levels = match args[i+1].parse::<u8>() {
                Ok(levels) => levels,
                Err(_) => panic!("Invalid number of levels: {}", args[i+1])
            };
            continue;
        }

        if (arg == "--sort" || arg == "-s") && args.len() > i + 1 {
            sort = match args[i+1].as_str() {
                "name" => Sort::NameASC,
                "size" => Sort::SizeDEC,
                "date" => Sort::DateASC,
                _ => panic!("Invalid sort type: {}", args[i+1])
            };

            if args.len() > i + 2 && !args[i+2].starts_with("-") {
                match args[i+2].to_lowercase().as_str() {
                    "asc" => sort = match sort {
                        Sort::NameDEC => Sort::NameASC,
                        Sort::SizeDEC => Sort::SizeASC,
                        Sort::DateDEC => Sort::DateASC,
                        _ => sort
                    },
                    "dec" => sort = match sort {
                        Sort::NameASC => Sort::NameDEC,
                        Sort::SizeASC => Sort::SizeDEC,
                        Sort::DateASC => Sort::DateDEC,
                        _ => sort
                    },
                    _ => panic!("Invalid sort order: {}", args[i+2])
                }
            }
        }
    }

    let dir_file = match get_files(&path, 1, &levels, &hidden, &verbose) {
        GetFile::File(dir_file) => dir_file,
        GetFile::Size(size) => {
            println!("{} ({})", path.display(), parse_file_size(size));
            exit(0);
        }
    };

    let sub_files = match dir_file.sub_files {
        Some(sub_files) => sub_files,
        None => {
            println!("{} ({})", path.display(), parse_file_size(dir_file.file_size));
            exit(0);
        }
    };

    print_sub_files(&sub_files, &sort);

    if !unicode_support() {
        println!("\n\nFile Folder Nomenclature:\n[D]: Directory\n[F]: File\n[L]: Link\n[?]: Unknown");
    }

    println!("DIR: {}\nSize: {}\nTime elapsed: {:?}", &path.display(), parse_file_size(dir_file.file_size), &start.elapsed());
}