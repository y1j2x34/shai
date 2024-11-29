use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let val = env::var("SHAI_OPENAI_API_BASE_URL");
    match val {
        Ok(v) => println!("base url: {}", v),
        Err(_) => println!("SHAI_OPENAI_API_BASE_URL not set"),
    }
    println!("Hello, world!");
}
