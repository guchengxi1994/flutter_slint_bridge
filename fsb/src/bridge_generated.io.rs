use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_say_hello(port_: i64) {
    wire_say_hello_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_event_loop(port_: i64) {
    wire_create_event_loop_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_show_auto_close_dialog(port_: i64, message: *mut wire_EventMessage) {
    wire_show_auto_close_dialog_impl(port_, message)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_event_message_0() -> *mut wire_EventMessage {
    support::new_leak_box_ptr(wire_EventMessage::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<(i8, i8)> for wire___record__i8_i8 {
    fn wire2api(self) -> (i8, i8) {
        (self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<EventMessage> for *mut wire_EventMessage {
    fn wire2api(self) -> EventMessage {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EventMessage>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}

impl Wire2Api<EventMessage> for wire_EventMessage {
    fn wire2api(self) -> EventMessage {
        EventMessage {
            alignment: self.alignment.wire2api(),
            title: self.title.wire2api(),
            content: self.content.wire2api(),
            dialog_type: self.dialog_type.wire2api(),
            duration_in_sec: self.duration_in_sec.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire___record__i8_i8 {
    field0: i8,
    field1: i8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EventMessage {
    alignment: wire___record__i8_i8,
    title: *mut wire_uint_8_list,
    content: *mut wire_uint_8_list,
    dialog_type: i32,
    duration_in_sec: *mut u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire___record__i8_i8 {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire___record__i8_i8 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EventMessage {
    fn new_with_null_ptr() -> Self {
        Self {
            alignment: Default::default(),
            title: core::ptr::null_mut(),
            content: core::ptr::null_mut(),
            dialog_type: Default::default(),
            duration_in_sec: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_EventMessage {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
