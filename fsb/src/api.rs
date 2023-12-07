use flutter_rust_bridge::StreamSink;

use crate::{event_loop::SEND_TO_DART_CONFIRM_STATUS_SINK, message::EventMessage};

pub fn say_hello() {
    println!("[rust] Hello, world!");
}

pub fn create_event_loop() {
    let _ = crate::event_loop::create_event_loop();
}

pub fn show_dialog(message: Option<EventMessage>) {
    crate::dialog::show_dialog(message);
}

pub fn confirm_status_stream(s: StreamSink<String>) -> anyhow::Result<()> {
    let mut stream = SEND_TO_DART_CONFIRM_STATUS_SINK.write().unwrap();
    *stream = Some(s);
    anyhow::Ok(())
}
