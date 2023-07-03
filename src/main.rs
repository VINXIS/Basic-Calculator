use crate::file::output_folder;

mod file;
mod help;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = std::env::current_dir().unwrap();
    let mut hidden = false;
    let mut levels: u8 = 1;
    for i in 0..args.len() {
        let arg = &args[i];
        if  arg == "--help" || arg == "-h" {
            help::print_help();
            std::process::exit(0);
        }

        if (arg == "--dir" || arg == "-d") && args.len() > i + 1 {
            // Get the next arg and validate the path
            let next_arg = &args[i+1];
            let next_path = std::path::Path::new(next_arg);
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

        if arg == "--show_hidden" || arg == "-s" {
            hidden = true;
            continue;
        }
    }


    println!("DIR: {}", &path.display());

    output_folder(&path, levels, 1, &hidden);
}