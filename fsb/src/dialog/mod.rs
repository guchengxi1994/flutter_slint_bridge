pub mod auto_close_dialog;
use std::sync::RwLock;

use lazy_static::lazy_static;
use slint::ComponentHandle;
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};

#[derive(Debug)]
pub enum MyEvent {
    CustomEvent(String),
}

lazy_static! {
    pub static ref PROXY: RwLock<Option<tao::event_loop::EventLoopProxy<MyEvent>>> =
        RwLock::new(None);
}

pub fn create_event_loop() {
    let mut builder: EventLoopBuilder<MyEvent> = EventLoopBuilder::<MyEvent>::with_user_event();
    #[cfg(target_os = "windows")]
    builder.with_any_thread(true);

    let event_loop: EventLoop<MyEvent> = builder.build();

    {
        let proxy: tao::event_loop::EventLoopProxy<MyEvent> = event_loop.create_proxy();
        let mut r = PROXY.write().unwrap();
        *r = Some(proxy);
    }

    let dialog = crate::dialog::auto_close_dialog::AutoCloseDialog::new().unwrap();
    dialog
        .window()
        .set_position(slint::PhysicalPosition::new(0, 0));

    event_loop.run(move |_event, _, _control_flow| {
        match _event {
            tao::event::Event::UserEvent(my_event) => match my_event {
                MyEvent::CustomEvent(message) => {
                    println!("Received custom event: {:?}", message);
                    // *control_flow = ControlFlow::Exit;

                    let _dialog_handle: slint::Weak<auto_close_dialog::AutoCloseDialog> =
                        dialog.as_weak();
                    slint::Timer::single_shot(std::time::Duration::from_secs(3), move || {
                        let _ = _dialog_handle.upgrade().unwrap().hide();
                    });

                    dialog.run().unwrap();
                }
            },
            _ => {}
        }
    });
}
