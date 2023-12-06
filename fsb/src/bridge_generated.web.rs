use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello(port_: MessagePort) {
    wire_say_hello_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_event_loop(port_: MessagePort) {
    wire_create_event_loop_impl(port_)
}

#[wasm_bindgen]
pub fn wire_show_auto_close_dialog(port_: MessagePort) {
    wire_show_auto_close_dialog_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
