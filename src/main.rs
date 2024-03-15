use std::error::Error;
use colour::{dark_green, yellow};
use newsapi::{*};

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=dd65e1957d654b369cd08e194690ae66";
    let articles = newsapi::get_articles(url)?;
    render_articles(&articles);
    Ok(())
}
