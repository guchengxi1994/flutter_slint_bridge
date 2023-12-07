use crate::message::EventMessage;

pub fn say_hello() {
    println!("[rust] Hello, world!");
}

pub fn create_event_loop() {
    let _ = crate::event_loop::create_event_loop();
}

pub fn show_notification(message: Option<EventMessage>) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        crate::dialog::notification::show_notification(message);
    });
}
