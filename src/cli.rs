use crate::read_movies;
use crate::MovieSortKey;
use clap::Parser;
use std::io::{self, Write};

fn _writeln(bytes: &[u8]) {
    io::stdout().write_all(bytes).unwrap_or_default();
    io::stdout().write_all(b"\n").unwrap_or_default();
}

#[derive(Parser)]
#[clap(name = "imdb-manager")]
struct Opts {
    #[clap(short, long, possible_values(["title", "year", "rating", "my-rating"]))]
    sort: Option<MovieSortKey>,
    #[clap(short, long)]
    reverse: bool,
}

pub fn run() {
    let opts = Opts::parse();
    let mut movies = read_movies(io::stdin());
    println!("Total: {}", movies.len());
    movies.sort(opts.sort.unwrap_or(MovieSortKey::Title), opts.reverse);
    movies.show();
}
