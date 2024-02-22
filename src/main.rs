use std::process;

use colored::*;
use structopt::StructOpt;

use cli::Opt;
use groups::generate_groups;

mod cli;
mod groups;

fn main() {
    let opt = Opt::from_args();

    match generate_groups(&opt.file_path, opt.n_groups) {
        Ok(groups) => {
            for (i, group) in groups.iter().enumerate() {
                print!("{}", format!("Group {}: ", i + 1).bold().blue());
                for name in group {
                    print!("{}, ", name.green());
                }
                println!(); // Optional: Add a newline for better separation between groups
            }
        }
        Err(e) => {
            eprintln!("Error processing file: {}", e.to_string().red());
            process::exit(1);
        }
    }
}
