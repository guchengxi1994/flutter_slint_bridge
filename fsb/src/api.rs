use crate::dialog::EventMessage;

pub fn say_hello() {
    println!("[rust] Hello, world!");
}

pub fn create_event_loop() {
    crate::dialog::create_event_loop();
}

pub fn show_auto_close_dialog(message: Option<EventMessage>) {
    println!("[rust] open dialog");
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        crate::dialog::auto_close_dialog::show_auto_close_dialog(message);
        println!("[rust] close dialog");
    });
}
