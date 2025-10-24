use std::env;
use clipboard::{ClipboardContext, ClipboardProvider};
use url_shortener::validate_url;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_url = &args[1];
    println!("generating short url for {}", input_url.clone());
    if validate_url(input_url.clone()).is_some() {
        let api_url = format!("https://tinyurl.com/api-create.php?url={}", 
                              urlencoding::encode(input_url));
        let client = reqwest::blocking::Client::new();
        let short_url = client.get(&api_url).send();
        
        match short_url {
            Ok(response) => {
                match response.text() {
                    Ok(url) => {
                        println!("short url generated> {}", url);
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(url).unwrap();
                        println!("url copied!");
                    }
                    Err(e) => {
                        eprintln!("failed to read response: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to generate short url: {:?}", e);
            }
        }
    } else {
        println!("invalid input url {}", input_url);
    }
}

