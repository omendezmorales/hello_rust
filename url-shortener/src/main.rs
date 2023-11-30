use std::env;
use url::{Host, Url};
use urlshortener::{client::UrlShortener, providers::Provider};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_url = &args[1];
    println!("generating short url for {}", input_url.clone());
    if validate_url(input_url.clone()).is_some() {
        let us = UrlShortener::new().unwrap();
        let short_url = us.generate(input_url, &Provider::IsGd);
        assert!(short_url.is_ok());
        println!("short url generated> {}", short_url.unwrap());
    } else {
        println!("invalid input url {}", input_url);
    }
}

fn validate_url(a_url: String) -> Option<bool> {
    let issue_list_url = Url::parse(&a_url).ok()?;
     Some(issue_list_url.scheme() == "https"
       && !issue_list_url.cannot_be_a_base())
    
}
