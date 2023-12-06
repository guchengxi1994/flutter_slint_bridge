pub mod auto_close_dialog;
use std::sync::RwLock;

use lazy_static::lazy_static;
use slint::ComponentHandle;
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};

#[derive(Debug)]
pub enum DialogType {
    Notification,
    ConfirmDialog,
    WarningDialog,
}

#[derive(Debug)]
pub struct EventMessage {
    pub alignment: (i8, i8),
    pub title: Option<String>,
    pub content: Option<String>,
    pub dialog_type: DialogType,
    pub duration_in_sec: Option<u8>,
}

impl EventMessage {
    pub fn default() -> Self {
        return EventMessage {
            alignment: (0, 0),
            title: Some("Default Dialog".to_string()),
            content: Some("Default content".to_string()),
            dialog_type: DialogType::Notification,
            duration_in_sec: Some(3),
        };
    }
}

lazy_static! {
    pub static ref PROXY: RwLock<Option<tao::event_loop::EventLoopProxy<EventMessage>>> =
        RwLock::new(None);
}

pub fn create_event_loop() {
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

    let dialog = crate::dialog::auto_close_dialog::AutoCloseDialog::new().unwrap();

    event_loop.run(move |_event, _, _control_flow| {
        match _event {
            tao::event::Event::UserEvent(my_event) => {
                println!("Received custom event: {:?}", my_event);
                // *control_flow = ControlFlow::Exit;

                let _dialog_handle: slint::Weak<auto_close_dialog::AutoCloseDialog> =
                    dialog.as_weak();

                match _dialog_handle.upgrade() {
                    Some(_handle) => {
                        let (width, height) = crate::utils::get_screen_size();

                        if width == -1 && height == -1 {
                            _handle
                                .window()
                                .set_position(slint::PhysicalPosition::new(0, 0));
                        } else {
                            match my_event.alignment {
                                (-1, -1) => {
                                    _handle
                                        .window()
                                        .set_position(slint::PhysicalPosition::new(0, 0));
                                }
                                (0, -1) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        (0.5 * width as f32) as i32- /*default width*/ 100,
                                        0,
                                    ));
                                }
                                (1, -1) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        width- /*default width*/ 100,
                                        0,
                                    ));
                                }
                                (-1, 0) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        0,
                                        (0.5 * height as f32) as i32 - /*default height*/ 50,
                                    ));
                                }
                                (0, 0) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        (0.5 * width as f32) as i32 - /*default width*/ 100,
                                        (0.5 * height as f32) as i32 - /*default height*/ 50,
                                    ));
                                }
                                (1, 0) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        width- /*default width*/ 100,
                                        (0.5 * height as f32) as i32 - /*default height*/ 50,
                                    ));
                                }
                                (-1, 1) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        0,
                                        height- /*default height*/ 50,
                                    ));
                                }
                                (0, 1) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        (0.5 * width as f32) as i32- /*default width*/ 100,
                                        height- /*default height*/ 50,
                                    ));
                                }
                                (1, 1) => {
                                    _handle.window().set_position(slint::PhysicalPosition::new(
                                        width- /*default width*/ 100,
                                        height- /*default height*/ 50,
                                    ));
                                }

                                _ => {
                                    _handle
                                        .window()
                                        .set_position(slint::PhysicalPosition::new(0, 0));
                                }
                            }
                        }

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
                    std::time::Duration::from_secs(my_event.duration_in_sec.unwrap_or(3).into()),
                    move || {
                        let _ = _dialog_handle.upgrade().unwrap().hide();
                    },
                );

                dialog.run().unwrap();
            }
            _ => {}
        }
    });
}
