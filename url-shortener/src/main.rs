use urlshortener::{client::UrlShortener,  providers::Provider};

fn main() {
    let us = UrlShortener::new().unwrap();
    let short_url = us.generate("https://1drv.ms/i/s!AsDcb1ONRmihnnRlWmRjGr7zVXAH?e=lXpanY", &Provider::IsGd );
    assert!(short_url.is_ok());
    println!("short url generated> {}",short_url.unwrap());
}
