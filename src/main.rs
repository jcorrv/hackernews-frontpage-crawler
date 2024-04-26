use reqwest;
use scraper::{Html, Selector};
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let hacker_news_url = "https://news.ycombinator.com";
    let client = reqwest::Client::new();

    println!("Fetching front page of hackernews: {}", hacker_news_url);

    loop {
        if let Ok(body) = fetch_page(&client, hacker_news_url).await {
            let urls = parse_for_links(&body);

            let url_output = urls.join("\n");

            println!("URLs fetched: {}", url_output);

            for url in urls {
                match client.get(&url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            println!("Successfully accessed: {}", url);
                        } else {
                            println!("Failed to access: {} with status: {}", url, response.status());
                        }
                    }
                    Err(e) => {
                        println!("Error accessing {}: {}", url, e);
                    }
                }
            }
        }

        println!("Done fetching frontpage of hackernews, waiting 30s and going again");
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

async fn fetch_page(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}

fn parse_for_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("td.title > span.titleline > a").unwrap();
    document.select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .map(|url| url.to_string())
        .collect()
}
