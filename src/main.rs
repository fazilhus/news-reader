use std::error::Error;
use newsapi;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=dd65e1957d654b369cd08e194690ae66";
    let articles = newsapi::get_articles(url)?;
    newsapi::render_articles(&articles);
    Ok(())
}
