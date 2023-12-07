use slint::ComponentHandle;
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};

use crate::{
    dialog::notification,
    message::{EventMessage, PROXY},
};

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

    event_loop.run(move |_event, _, _control_flow| {
        match _event {
            tao::event::Event::UserEvent(my_event) => {
                println!("Received custom event: {:?}", my_event);
                // *control_flow = ControlFlow::Exit;

                let _notification_handle: slint::Weak<notification::Notification> =
                    notification.as_weak();

                match _notification_handle.upgrade() {
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
                        let _ = _notification_handle.upgrade().unwrap().hide();
                    },
                );

                notification.run().unwrap();
            }
            _ => {}
        }
    });
}
