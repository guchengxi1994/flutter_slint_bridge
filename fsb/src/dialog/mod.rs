use crate::message::{EventMessage, PROXY};

pub mod about_dialog;
pub mod confirm_dialog;
pub mod notification;
pub mod warning_dialog;

pub fn show_dialog(message: Option<EventMessage>) {
    let r = PROXY.read().unwrap();

    match message {
        Some(m) => {
            let _ = r.clone().unwrap().send_event(m);
        }
        None => {
            let _ = r.clone().unwrap().send_event(EventMessage::default());
        }
    }
}
