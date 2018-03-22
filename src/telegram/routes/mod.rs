mod actions;

extern crate telegram_bot;    
use self::telegram_bot::*;

pub fn resolve (command: &str, message: Message) -> String {
    match command {
    "/mycellisonfire" => actions::isonfire::run(message.from.first_name),
    "/batata" =>  "dlc".to_string(),        
    _ =>  "".to_string(),
    }
}