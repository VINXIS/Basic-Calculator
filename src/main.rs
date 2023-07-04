use std::{env::{args, current_dir}, process::exit, path::Path};

use crate::{file::output_path, util::{unicode_support, parse_file_size}};

mod file;
mod util;

fn print_help() {
    println!("Usage: treedir [OPTIONS]
    -d, --dir <PATH>    The directory to list (Default: current directory)
    -l, --levels <NUM>  The number of levels to list (Default: 1)
    -s, --show_hidden   Show hidden files and folders
    -h, --help          Prints help information");
}

fn main() {
    let args: Vec<String> = args().collect();
    let hidden = args.contains(&String::from("--show_hidden")) || args.contains(&String::from("-s"));
    let mut path = current_dir().unwrap_or_default();
    let mut levels: u8 = 1;
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
            levels = args[i+1].parse::<u8>().unwrap();
            continue;
        }
    }

    println!("DIR: {}", &path.display());

    let mut counter: u32 = 0;
    let mut size_count: u64 = 0;
    output_path(&path, levels, 1, &hidden, &mut counter, &mut size_count);

    println!("{} items found\n{} total directory size", counter, parse_file_size(size_count));

    if !unicode_support() {
        println!("\n\nFile Folder Nomenclature:\n[D]: Directory\n[F]: File\n[L]: Link\n[?]: Unknown");
    }
}