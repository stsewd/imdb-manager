use crate::read_movies;
use crate::MovieSortKey;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};

fn _writeln(bytes: &[u8]) {
    io::stdout().write_all(bytes).unwrap_or_default();
    io::stdout().write_all(b"\n").unwrap_or_default();
}

#[derive(Parser, Debug)]
#[clap(name = "imdb-manager", version=clap::crate_version!())]
struct Opts {
    #[clap(short, long)]
    #[clap(possible_values(["title", "year", "rating", "my-rating"]))]
    #[clap(default_value("title"))]
    sort: MovieSortKey,

    #[clap(short, long)]
    reverse: bool,

    #[clap(about("Path to the CSV file exported from IMDb"))]
    file: Option<String>,
}

pub fn run() {
    let opts = Opts::parse();
    let mut movies = if let Some(file) = opts.file {
        let file = File::open(file).expect("File not found");
        read_movies(BufReader::new(file))
    } else {
        read_movies(io::stdin())
    };
    println!("Total: {}", movies.len());
    movies.sort(opts.sort, opts.reverse);
    movies.show();
}
