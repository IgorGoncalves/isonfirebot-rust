extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod telegram;

fn main() {
    
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();    
    telegram::telegram::init(token);    

}

