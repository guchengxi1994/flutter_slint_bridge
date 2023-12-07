use std::sync::{Mutex, RwLock};

use flutter_rust_bridge::StreamSink;
use slint::ComponentHandle;
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};

use crate::{
    dialog::notification,
    message::{DialogType, EventMessage, PROXY},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref DIALOG_SHOW: Mutex<bool> = Mutex::new(false);
}

pub static SEND_TO_DART_CONFIRM_STATUS_SINK: RwLock<Option<StreamSink<String>>> = RwLock::new(None);

pub fn create_event_loop() -> anyhow::Result<()> {
    let mut builder: EventLoopBuilder<EventMessage> =
        EventLoopBuilder::<EventMessage>::with_user_event();
    #[cfg(target_os = "windows")]
    builder.with_any_thread(true);

    let event_loop: EventLoop<EventMessage> = builder.build();

    {
        let proxy: tao::event_loop::EventLoopProxy<EventMessage> = event_loop.create_proxy();
        let mut r = PROXY.write().unwrap();
        *r = Some(proxy);
    }

    let notification = crate::dialog::notification::Notification::new().unwrap();
    let confirm = crate::dialog::confirm_dialog::ConfirmDialog::new().unwrap();

    confirm.on_close_dialog(|| {
        {
            let mut r = DIALOG_SHOW.lock().unwrap();
            *r = false;
        }
        match SEND_TO_DART_CONFIRM_STATUS_SINK.try_read() {
            Ok(s) => match s.as_ref() {
                Some(s0) => {
                    s0.add("aaaa".to_string());
                }
                None => {
                    println!("[rust-error] Stream is None");
                }
            },
            Err(_) => {
                println!("[rust-error] Stream read error");
            }
        }
    });

    event_loop.run(move |_event, _, _control_flow| {
        match _event {
            tao::event::Event::UserEvent(my_event) => {
                println!("Received custom event: {:?}", my_event);
                // *control_flow = ControlFlow::Exit;

                let _notification_handle: slint::Weak<notification::Notification> =
                    notification.as_weak();
                let _confirm_dialog_handle = confirm.as_weak();

                {
                    let mut r = DIALOG_SHOW.lock().unwrap();
                    *r = true;
                }

                match my_event.dialog_type {
                    DialogType::Notification => {
                        match _notification_handle.upgrade() {
                            Some(_handle) => {
                                let position = get_position(my_event.alignment);
                                _handle.window().set_position(slint::PhysicalPosition::new(
                                    position.0, position.1,
                                ));

                                if let Some(s) = my_event.title {
                                    _handle.set_window_title(s.into());
                                }
                                if let Some(s) = my_event.content {
                                    _handle.set_content(s.into());
                                }
                            }
                            None => {}
                        }
                        slint::Timer::single_shot(
                            std::time::Duration::from_secs(
                                my_event.duration_in_sec.unwrap_or(3).into(),
                            ),
                            move || {
                                {
                                    let mut r = DIALOG_SHOW.lock().unwrap();
                                    *r = false;
                                }
                                let _ = _notification_handle.upgrade().unwrap().hide();
                            },
                        );

                        notification.run().unwrap();
                    }
                    DialogType::ConfirmDialog => {
                        match _confirm_dialog_handle.upgrade() {
                            Some(_handle) => {
                                let position = get_position(my_event.alignment);
                                _handle.window().set_position(slint::PhysicalPosition::new(
                                    position.0, position.1,
                                ));

                                if let Some(s) = my_event.title {
                                    _handle.set_window_title(s.into());
                                }
                                if let Some(s) = my_event.content {
                                    _handle.set_content(s.into());
                                }
                            }
                            None => {}
                        }
                        confirm.run().unwrap();
                    }
                    DialogType::WarningDialog => {}
                }
            }
            _ => {}
        }
    });
}

#[allow(dead_code)]
pub fn could_show_more() -> bool {
    let r = DIALOG_SHOW.lock().unwrap();
    return *r;
}

fn get_position(alignment: (i8, i8)) -> (i32, i32) {
    let (width, height) = crate::utils::get_screen_size();
    if width == -1 && height == -1 {
        return (0, 0);
    }

    match alignment {
        (-1, -1) => (0, 0),
        (0, -1) => ((0.5 * width as f32) as i32- /*default width*/ 100, 0),

        (1, -1) => (width- /*default width*/ 100, 0),
        (-1, 0) => (0, (0.5 * height as f32) as i32 - /*default height*/ 50),
        (0, 0) => (
            (0.5 * width as f32) as i32 - /*default width*/ 100,
            (0.5 * height as f32) as i32 - /*default height*/ 50,
        ),
        (1, 0) => (
            width- /*default width*/ 100,
            (0.5 * height as f32) as i32 - /*default height*/ 50,
        ),
        (-1, 1) => (0, height- /*default height*/ 50),
        (0, 1) => (
            (0.5 * width as f32) as i32- /*default width*/ 100,
            height- /*default height*/ 50,
        ),
        (1, 1) => (width- /*default width*/ 100, height- /*default height*/ 50),

        _ => (0, 0),
    }
}
