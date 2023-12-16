use std::sync::RwLock;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum DialogType {
    Notification,
    ConfirmDialog,
    WarningDialog,
    AboutDialog,
    SubWindow,
}

#[derive(Debug, Clone)]
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
