use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    // parse a -f/--file argument to get target file
    file: String,
    // extra vec to collect misc arguments passed in
    args: Vec<String>,
}

fn main() {
    // parse cli arguments
    let args = Args::parse();
    // parse the target file from arguments
    let file = PathBuf::from(args.file);

    // read file to this string
    let file = fs::read_to_string(file).expect("File not found lol");

    // any extra args just consider as target substrings to search for
    for arg in args.args {
        // if substring found
        if file.contains(&arg) {
            println!("Found: {arg}");
        } else {
            // substring not found
            println!("Did not find: {arg}")
        }
    }
}
