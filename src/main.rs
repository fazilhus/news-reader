use std::error::Error;
use dotenv::dotenv;

use newsapi::{*};

mod theme;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines!\n\n");
    for article in articles {
        theme.print_text(&format!("'{}'", article.title()));
        theme.print_text(&format!("> *{}*", article.url()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv().ok();

    let api_key = std::env::var("API_KEY")?;
    
    let binding = NewsAPI::new(&api_key);
    let mut newsapi = binding;
    newsapi
        .endpoint(EndPoint::TopHeadlines)
        .country(Country::Us); 

    let newsapi_response = newsapi.fetch_async().await?;
    //let newsapi_response = newsapi.fetch()?;
    render_articles(newsapi_response.articles());
    Ok(())
}
