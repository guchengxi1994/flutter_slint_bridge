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
pub fn wire_show_dialog(port_: MessagePort, message: JsValue) {
    wire_show_dialog_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_dart_message_stream(port_: MessagePort) {
    wire_dart_message_stream_impl(port_)
}

#[wasm_bindgen]
pub fn wire_send_dart_message(port_: MessagePort, message: String) {
    wire_send_dart_message_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_set_item_id(port_: MessagePort, id: i32) {
    wire_set_item_id_impl(port_, id)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<(i8, i8)> for JsValue {
    fn wire2api(self) -> (i8, i8) {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        (self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

impl Wire2Api<EventMessage> for JsValue {
    fn wire2api(self) -> EventMessage {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            6,
            "Expected 6 elements, got {}",
            self_.length()
        );
        EventMessage {
            alignment: self_.get(0).wire2api(),
            title: self_.get(1).wire2api(),
            content: self_.get(2).wire2api(),
            dialog_type: self_.get(3).wire2api(),
            duration_in_sec: self_.get(4).wire2api(),
            data: self_.get(5).wire2api(),
        }
    }
}

impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<DialogType> for JsValue {
    fn wire2api(self) -> DialogType {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
