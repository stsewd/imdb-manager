use serde::Deserialize;
use std::io::Read;
use std::str::FromStr;

pub mod cli;

#[derive(Debug, Deserialize)]
struct Movie {
    #[serde(rename = "Const")]
    id: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Title Type")]
    type_: String,
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "IMDb Rating")]
    rating: Option<f32>,
    #[serde(rename = "Year")]
    year: i32,
    #[serde(rename = "Genres")]
    genres: String,
    #[serde(rename = "Runtime (mins)")]
    runtime: String,
    #[serde(rename = "Directors")]
    directors: String,
    #[serde(rename = "Your Rating")]
    my_rating: Option<f32>,
}

impl Movie {
    fn show(&self) {
        let mut detail = format!("{} ({})", self.title, self.year);
        if let Some(rating) = self.rating {
            if let Some(my_rating) = self.my_rating {
                detail = format!("{} - {}/{}", detail, rating, my_rating);
            } else {
                detail = format!("{} - {}", detail, rating);
            }
        }
        println!("{} {}", detail, self.url);
    }
}

#[derive(Debug)]
pub enum MovieSortKey {
    Title,
    Rating,
    MyRating,
    Year,
}

impl FromStr for MovieSortKey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "year" => Ok(MovieSortKey::Year),
            "title" => Ok(MovieSortKey::Title),
            "rating" => Ok(MovieSortKey::Rating),
            "my-rating" => Ok(MovieSortKey::MyRating),
            _ => Err("No match"),
        }
    }
}

#[derive(Debug)]
struct MovieList {
    data: Vec<Movie>,
}

impl MovieList {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    fn push(&mut self, movie: Movie) {
        self.data.push(movie);
    }

    fn sort(&mut self, key: MovieSortKey, reverse: bool) {
        self.data.sort_by(|a, b| -> std::cmp::Ordering {
            let (a, b) = if reverse { (b, a) } else { (a, b) };
            let default_value = if reverse { 0.0 } else { 99.99 };
            match key {
                MovieSortKey::Title => a.title.cmp(&b.title),
                MovieSortKey::Rating => a
                    .rating
                    .unwrap_or(default_value)
                    .partial_cmp(&b.rating.unwrap_or(default_value))
                    .unwrap(),
                MovieSortKey::MyRating => a
                    .my_rating
                    .unwrap_or(default_value)
                    .partial_cmp(&b.my_rating.unwrap_or(default_value))
                    .unwrap(),
                MovieSortKey::Year => a.year.cmp(&b.year),
            }
        });
    }

    fn iter(&self) -> std::slice::Iter<Movie> {
        self.data.iter()
    }

    fn show(&self) {
        for movie in self.iter() {
            movie.show();
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

fn read_movies<R: Read>(reader: R) -> MovieList {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut records = MovieList::new();
    for result in csv_reader.deserialize() {
        let record: Movie = result.unwrap();
        records.push(record);
    }
    records
}
