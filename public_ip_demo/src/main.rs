use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Perform my public IP address lookup
    let result = public_ip_address::perform_lookup(None).await?;
    println!("{}", result);
    Ok(())
}


