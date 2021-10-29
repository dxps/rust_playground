mod theme;

use std::error::Error;

use api_news::{Article, Country, NewsAPI};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Loading .env settings into the env.
    dotenv().ok();

    let api_key = std::env::var("API_KEY")?;

    let mut news_api = NewsAPI::new(&api_key);
    news_api
        .endpoint(api_news::Endpoint::TopHeadlines)
        .country(Country::US);

    let news_response = news_api.fetch_async().await?;

    print_articles(news_response.articles());

    Ok(())
}

fn print_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for a in articles {
        theme.print_text(&format!("`{}`", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---");
    }
}
