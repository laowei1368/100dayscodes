use clap::{Parser,CommandFactory};
use is_terminal::IsTerminal as _;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};
use std::io::StdinLock;

/// Count the number of lines in a file
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file to read
    file: PathBuf,
}
fn main() {
    let args = Cli::parse();
    let mut word_count = 0;
    let mut file = args.file;

    if file == PathBuf::from("-"){
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }
        file = PathBuf::from("<stdin>");
        word_count = word_in_buf_reader(BufReader::new(stdin().lock()));
    } else {
        word_count = word_in_buf_reader(BufReader::new(File::open(&file).unwrap()));
    }

    println!("Words in {}: {}", file.to_str().unwrap(),word_count);
}

fn word_in_buf_reader<R: BufRead>(buf_reader: R) -> usize {
    let mut count = 0;
    for line in buf_reader.lines() {
         count += line.unwrap().split(' ').count()
    }
    count
}