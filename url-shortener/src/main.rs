use std::env;
use std::process::Command;
use arboard::{Clipboard, SetExtLinux};
use url_shortener::validate_url;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Clipboard-holder mode: spawned as a detached child to keep contents alive
    if args.get(1).map(|s| s.as_str()) == Some("--clipboard-hold") {
        let text = args.get(2).cloned().unwrap_or_default();
        let mut ctx = Clipboard::new().unwrap();
        ctx.set().wait().text(text).unwrap();
        return;
    }

    let input_url = &args[1];
    println!("generating short url for {}", input_url);
    if validate_url(input_url.clone()).is_some() {
        let api_url = format!(
            "https://tinyurl.com/api-create.php?url={}",
            urlencoding::encode(input_url)
        );
        let client = reqwest::blocking::Client::new();
        match client.get(&api_url).send() {
            Ok(response) => match response.text() {
                Ok(url) => {
                    println!("short url generated> {}", url);
                    // Spawn a detached child that holds the clipboard until something reads it
                    let exe = env::current_exe().unwrap();
                    Command::new(exe)
                        .arg("--clipboard-hold")
                        .arg(&url)
                        .spawn()
                        .expect("failed to spawn clipboard holder");
                    println!("url copied!");
                }
                Err(e) => eprintln!("failed to read response: {:?}", e),
            },
            Err(e) => eprintln!("failed to generate short url: {:?}", e),
        }
    } else {
        println!("invalid input url {}", input_url);
    }
}
