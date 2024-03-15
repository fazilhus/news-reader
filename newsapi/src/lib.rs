use std::error::Error;
use colour::{dark_green, yellow};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Articles {
    articles: Vec<Article>,
}


#[derive(Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}


pub fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

pub fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}
