pub mod model;
pub mod pin_window_wrapper;
pub mod utils;

use crate::{
    dialog::about_dialog,
    event_loop::{model::InputPinWindowItemList, pin_window_wrapper::pin_window_handle_wrapper},
    ui::PinWindow,
};
use std::{
    sync::{mpsc::Sender, Mutex, RwLock},
    time::Duration,
};

use flutter_rust_bridge::StreamSink;
use slint::ComponentHandle;
use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows,
};
// use tray_item::{IconSource, TrayItem};

use crate::{
    dialog::notification,
    event_loop::{model::PinWindowItem, utils::get_position},
    message::{DialogType, EventMessage, PROXY},
};

use lazy_static::lazy_static;

use self::model::Rust2DartResponse;

use std::sync::mpsc::channel;

lazy_static! {
    static ref DIALOG_SHOW: Mutex<bool> = Mutex::new(false);
    pub static ref MP_SENDER: Mutex<Option<Sender<String>>> = Mutex::new(None);
    pub static ref PIN_WINDOW_ITEMS: RwLock<Vec<PinWindowItem>> = RwLock::new(Vec::new());
}

pub static SEND_TO_DART_MESSAGE_SINK: RwLock<Option<StreamSink<String>>> = RwLock::new(None);

#[allow(unused_assignments)]
pub fn create_event_loop() -> anyhow::Result<()> {
    let mut builder: EventLoopBuilder<EventMessage> =
        EventLoopBuilder::<EventMessage>::with_user_event();
    #[cfg(target_os = "windows")]
    builder.with_any_thread(true);

    let event_loop: EventLoop<EventMessage> = builder.build();
    let (_tx, rx) = channel::<String>();

    // let tr = TrayItem::new("Weaving", IconSource::Resource("name-of-icon-in-rc-file"));
    // if let Ok(mut tray) = tr {
    //     let about_tx = _tx.clone();
    //     tray.add_menu_item("关于", move || {
    //         about_tx.send("navigate".to_owned()).unwrap();
    //     })
    //     .unwrap();
    //     tray.inner_mut().add_separator().unwrap();

    //     let quit_tx = _tx.clone();
    //     tray.add_menu_item("Quit", move || {
    //         quit_tx.send("quit".to_owned()).unwrap();
    //     })
    //     .unwrap();
    // } else {
    //     println!("[rust] tray error : {:?}", tr.err());
    // }

    // tray.add_label("Tray Label").unwrap();

    {
        let mut r = MP_SENDER.lock().unwrap();
        *r = Some(_tx.clone());
    }

    {
        let proxy: tao::event_loop::EventLoopProxy<EventMessage> = event_loop.create_proxy();
        let mut r = PROXY.write().unwrap();
        *r = Some(proxy);
    }

    std::thread::spawn(move || loop {
        let _ = _tx.clone().send("avoid drop reciever".to_owned());
        std::thread::sleep(Duration::from_secs(1));
    });

    let notification = crate::dialog::notification::Notification::new().unwrap();
    let confirm = crate::dialog::confirm_dialog::ConfirmDialog::new().unwrap();
    let pin_win = PinWindow::new().unwrap();

    // pin_win = pin_window_wrapper(pin_win);

    let mut pin_win_handle = pin_win.as_weak();

    std::thread::spawn(move || loop {
        let s = rx.recv();
        if let Ok(_s) = s {
            if _s == "navigate" {
                crate::dialog::show_dialog(Some(EventMessage {
                    alignment: (0, 0),
                    title: None,
                    content: None,
                    dialog_type: DialogType::AboutDialog,
                    duration_in_sec: None,
                    data: None,
                }));
            }
            if _s == "quit" {
                crate::dialog::show_dialog(Some(EventMessage {
                    alignment: (0, 0),
                    title: Some("".to_owned()),
                    content: Some("Quit...".to_string()),
                    dialog_type: DialogType::Notification,
                    duration_in_sec: None,
                    data: None,
                }));
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_secs(3));
                    std::process::exit(0);
                });
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    });

    let confirm_dialog_handle = confirm.as_weak();

    confirm.on_close_dialog(move || {
        {
            let mut r = DIALOG_SHOW.lock().unwrap();
            *r = false;
        }
        match SEND_TO_DART_MESSAGE_SINK.try_read() {
            Ok(s) => match s.as_ref() {
                Some(s0) => {
                    let b = Rust2DartResponse::<bool> { data: true };
                    s0.add(b.to_json());
                }
                None => {
                    println!("[rust-error] Stream is None");
                }
            },
            Err(_) => {
                println!("[rust-error] Stream read error");
            }
        }
        let _ = confirm_dialog_handle.upgrade().unwrap().hide();
    });

    let about = about_dialog::AboutDialog::new()?;

    let about_dialog_handle = about.as_weak();

    about.on_navigate(move || {
        let _ = open::that("https://github.com/guchengxi1994");
        let _ = about_dialog_handle.upgrade().unwrap().hide();
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
                    DialogType::AboutDialog => {
                        about.run().unwrap();
                    }
                    DialogType::SubWindow => {
                        if let Some(s) = my_event.title {
                            pin_win_handle.upgrade().unwrap().set_title_name(s.into());
                        }

                        let r = InputPinWindowItemList::from_str(
                            my_event.data.unwrap_or("".to_string()),
                        );

                        match r {
                            Ok(_r) => {
                                pin_win_handle =
                                    pin_window_handle_wrapper(pin_win_handle.clone(), _r);
                            }
                            Err(_) => {
                                pin_win_handle =
                                    pin_window_handle_wrapper(pin_win_handle.clone(), vec![]);
                            }
                        }

                        pin_win.run().unwrap();
                    }
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
