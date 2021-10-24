use crate::read_movies;
use crate::MovieSortKey;
use std::io::{self, Write};

fn _writeln(bytes: &[u8]) {
    io::stdout().write_all(bytes).unwrap_or_default();
    io::stdout().write_all(b"\n").unwrap_or_default();
}

pub fn run() {
    let mut movies = read_movies(io::stdin());
    println!("Total: {}", movies.len());
    movies.sort(MovieSortKey::MyRating, false);
    movies.show();
}
