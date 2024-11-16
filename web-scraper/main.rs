use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::io::stdin;

fn main() {
    println!("Type website url to scrape: ");
    let mut site_url: String = String::new();
    stdin()
        .read_line(&mut site_url)
        .expect("Expected a url..?");    
    scrape_this_website(site_url);
}

fn scrape_this_website(website_url: String) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let url: String = website_url;
    let response = client.get(url).send()?.text()?;

    let document = Html::parse_document(&response);

    let mut tag_to_parse: String = String::new();

    println!("Type tag to collect: ");
    stdin()
        .read_line(&mut tag_to_parse)
        .expect("Expected a url..?"); 
    let selector = Selector::parse(&tag_to_parse).unwrap();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("Found following text:");
        println!("{}", text);
    }

    Ok(())
}



/*
fn scraper_function() -> Result<(), Box<dyn Error>> {
    // Step 1: Initialize HTTP client
    let client = Client::new();

    // Step 2: Make a GET request to the website
    let url = "https://example.com";
    let response = client.get(url).send()?.text()?;

    // Step 3: Parse the HTML response
    let document = Html::parse_document(&response);

    // Step 4: Define a CSS selector to scrape specific data
    let selector = Selector::parse("h1").unwrap();

    // Step 5: Extract and print the data
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("Found: {}", text);
    }

    Ok(())
}

 */