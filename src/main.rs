use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("RESEND_API_KEY").expect("API KEY not found");
    println!("API KEY: {}", api_key);
}
