use flutter_rust_bridge::StreamSink;

use crate::{event_loop::SEND_TO_DART_MESSAGE_SINK, message::EventMessage};

pub fn say_hello() {
    println!("[rust] Hello, world!");
}

pub fn create_event_loop() {
    let _ = crate::event_loop::create_event_loop();
}

pub fn show_dialog(message: Option<EventMessage>) {
    crate::dialog::show_dialog(message);
}

pub fn dart_message_stream(s: StreamSink<String>) -> anyhow::Result<()> {
    let mut stream = SEND_TO_DART_MESSAGE_SINK.write().unwrap();
    *stream = Some(s);
    anyhow::Ok(())
}

pub fn send_dart_message(message: String) {
    let r = crate::event_loop::utils::send_dart_message(message);
    match r {
        Ok(_) => {}
        Err(e) => {
            println!("[rust] error :{:?}", e);
        }
    }
}
