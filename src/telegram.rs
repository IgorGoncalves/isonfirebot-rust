pub mod telegram {
    extern crate futures;
    extern crate telegram_bot;
    extern crate tokio_core;

    use self::futures::Stream;
    use self::tokio_core::reactor::Core;
    use self::telegram_bot::*;

    fn process(api: Api, message: Message) {
        if let MessageKind::Text { ref data, .. } = message.kind {
            match data.as_str() {
                "/mycellisonfire" => api.spawn(message.text_reply(
                    format!("No, {}! Your cell is not on fire... I think", &message.from.first_name)
                )),
                "/batata" =>  api.spawn(message.text_reply(
                    "frita".to_string()
                )),
                _ =>  api.spawn(message.text_reply(
                ""
                )),
            }
        }
    }

    pub fn init(token: String) {        
        
        let mut core = Core::new().unwrap();
        
        let api = Api::configure(token).build(core.handle()).unwrap();

        // Fetch new updates via long poll method
        let future = api.stream().for_each(|update| {
            
            if let UpdateKind::Message(message) = update.kind {
                process(api.clone(), message)
            }

            Ok(())
        });

        core.run(future).unwrap();
    }
}