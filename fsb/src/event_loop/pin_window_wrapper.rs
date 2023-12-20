use std::rc::Rc;
use std::sync::Mutex;
use std::time::Duration;

use crate::ui::ListViewItem;
use crate::ui::PinWindow;
use lazy_static::lazy_static;
use serde::Serialize;
use slint::ComponentHandle;
use slint::Weak;

use super::model::PinWindowItem;
use super::{model::Rust2DartResponse, SEND_TO_DART_MESSAGE_SINK};

lazy_static! {
    pub static ref ITEM_ID: Mutex<Option<i32>> = Mutex::new(None);
}

#[derive(Serialize, Debug)]
struct Request {
    pub title: String,
    #[serde(rename = "type")]
    pub _type: i16,
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn pin_window_wrapper(pin_win: PinWindow, inited_items: Vec<PinWindowItem>) -> PinWindow {
    let pin_win_clone = pin_win.as_weak();

    // let inited_items;

    // {
    //     inited_items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
    // }

    let todo_model = Rc::new(slint::VecModel::<ListViewItem>::default());
    for i in inited_items {
        todo_model.push(ListViewItem {
            checked: i.checked,
            title: i.title.into(),
            id: i.id,
        });
    }

    pin_win
        .window()
        .set_position(slint::LogicalPosition::new(0., 0.));

    pin_win.on_re_sync({
        let _todo_model = todo_model.clone();
        let _pin_win_clone = pin_win_clone.unwrap();
        move || {
            // handle.upgrade().unwrap().set_todo_model(todo_model.clone().into())
            // TODO 告诉dart重新同步
            // let items;

            // {
            //     items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
            // }
            // println!("length : {:?}", items.len());
            // loop {
            //     if todo_model.row_count() == 0 {
            //         break;
            //     }
            //     todo_model.remove(0);
            // }
            // // todo_model = Rc::new(slint::VecModel::<ListViewItem>::default());
            // for i in items {
            //     todo_model.push(ListViewItem {
            //         checked: i.checked,
            //         title: i.title.into(),
            //         id: i.id,
            //     });
            // }
            // for i in 0..todo_model.row_count() {
            //     println!("{:?}", todo_model.row_data(i));
            // }

            // pin_win_clone.set_todo_model(todo_model.clone().into());
        }
    });

    pin_win.on_mouse_move({
        let pin_win_clone = pin_win_clone.unwrap();
        move |delta_x, delta_y| {
            let logical_pos = pin_win_clone
                .window()
                .position()
                .to_logical(pin_win_clone.window().scale_factor());
            pin_win_clone
                .window()
                .set_position(slint::LogicalPosition::new(
                    logical_pos.x + delta_x,
                    logical_pos.y + delta_y,
                ));
        }
    });

    pin_win.on_close_window({
        let pin_win_clone = pin_win_clone.unwrap();
        move || {
            let _ = pin_win_clone.hide();
        }
    });

    pin_win.on_todo_added({
        let todo_model = todo_model.clone();
        let pin_win_clone = pin_win_clone.unwrap();
        move |str| {
            match SEND_TO_DART_MESSAGE_SINK.try_read() {
                Ok(s) => match s.as_ref() {
                    Some(s0) => {
                        let b = Rust2DartResponse::<Request> {
                            data: Request {
                                title: str.to_string(),
                                _type: 1,
                            },
                        };
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
            // [TODO] 这里其实要等 flutter 端的返回
            // 创建一个变量，loop等待变量值发生改变
            pin_win_clone.on_show_loading(|| {});
            let mut i;
            loop {
                std::thread::sleep(Duration::from_millis(50));
                {
                    i = ITEM_ID.lock().unwrap().clone();
                }
                if i.is_some() {
                    break;
                }
            }

            // 然后把这个 item 塞到 PIN_WINDOW_ITEMS 中
            todo_model.push(ListViewItem {
                checked: false,
                title: str,
                id: i.unwrap(),
            });
            // for i in 0..todo_model.row_count() {
            //     println!("{:?}", todo_model.row_data(i));
            // }

            pin_win_clone.on_close_loading(|| {});

            {
                let mut i = ITEM_ID.lock().unwrap();
                *i = None;
            }
        }
    });

    pin_win.on_item_status_changed(move |b, index| println!("{}-{}", index, b));

    pin_win.set_todo_model(todo_model.into());

    pin_win
}

#[allow(unused_assignments)]
pub fn pin_window_handle_wrapper(
    pin_win_handle: Weak<PinWindow>,
    inited_items: Vec<PinWindowItem>,
) -> Weak<PinWindow> {
    let pin_win_clone = pin_win_handle.clone();

    // let inited_items;

    // {
    //     inited_items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
    // }

    let todo_model = Rc::new(slint::VecModel::<ListViewItem>::default());
    for i in inited_items {
        todo_model.push(ListViewItem {
            checked: i.checked,
            title: i.title.into(),
            id: i.id,
        });
    }

    pin_win_handle
        .upgrade()
        .unwrap()
        .window()
        .set_position(slint::LogicalPosition::new(0., 0.));

    pin_win_handle.upgrade().unwrap().on_re_sync({
        let _todo_model = todo_model.clone();
        let _pin_win_clone = pin_win_clone.unwrap();
        move || {
            // handle.upgrade().unwrap().set_todo_model(todo_model.clone().into())
            // TODO 告诉dart重新同步
            // let items;

            // {
            //     items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
            // }
            // println!("length : {:?}", items.len());
            // loop {
            //     if todo_model.row_count() == 0 {
            //         break;
            //     }
            //     todo_model.remove(0);
            // }
            // // todo_model = Rc::new(slint::VecModel::<ListViewItem>::default());
            // for i in items {
            //     todo_model.push(ListViewItem {
            //         checked: i.checked,
            //         title: i.title.into(),
            //         id: i.id,
            //     });
            // }
            // for i in 0..todo_model.row_count() {
            //     println!("{:?}", todo_model.row_data(i));
            // }

            // pin_win_clone.set_todo_model(todo_model.clone().into());
        }
    });

    pin_win_handle.upgrade().unwrap().on_mouse_move({
        let pin_win_clone = pin_win_clone.unwrap();
        move |delta_x, delta_y| {
            let logical_pos = pin_win_clone
                .window()
                .position()
                .to_logical(pin_win_clone.window().scale_factor());
            pin_win_clone
                .window()
                .set_position(slint::LogicalPosition::new(
                    logical_pos.x + delta_x,
                    logical_pos.y + delta_y,
                ));
        }
    });

    pin_win_handle.upgrade().unwrap().on_close_window({
        let pin_win_clone = pin_win_clone.unwrap();
        move || {
            let _ = pin_win_clone.hide();
        }
    });

    pin_win_handle.upgrade().unwrap().on_todo_added({
        let todo_model = todo_model.clone();
        let pin_win_clone = pin_win_clone.unwrap();
        move |str| {
            match SEND_TO_DART_MESSAGE_SINK.try_read() {
                Ok(s) => match s.as_ref() {
                    Some(s0) => {
                        let b = Rust2DartResponse::<Request> {
                            data: Request {
                                title: str.to_string(),
                                _type: 1,
                            },
                        };
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
            // [TODO] 这里其实要等 flutter 端的返回
            // 创建一个变量，loop等待变量值发生改变
            pin_win_clone.on_show_loading(|| {});
            let mut i;
            loop {
                std::thread::sleep(Duration::from_millis(50));
                {
                    i = ITEM_ID.lock().unwrap().clone();
                }
                if i.is_some() {
                    break;
                }
            }

            // 然后把这个 item 塞到 PIN_WINDOW_ITEMS 中
            todo_model.push(ListViewItem {
                checked: false,
                title: str,
                id: i.unwrap(),
            });
            // for i in 0..todo_model.row_count() {
            //     println!("{:?}", todo_model.row_data(i));
            // }

            pin_win_clone.on_close_loading(|| {});

            {
                let mut i = ITEM_ID.lock().unwrap();
                *i = None;
            }
        }
    });

    pin_win_handle.upgrade().unwrap().on_focus_main(move || {
        match SEND_TO_DART_MESSAGE_SINK.try_read() {
            Ok(s) => match s.as_ref() {
                Some(s0) => {
                    let _b = Rust2DartResponse::<Request> {
                        data: Request {
                            title: "".to_string(),
                            _type: 3,
                        },
                    };
                    s0.add(_b.to_json());
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

    pin_win_handle
        .upgrade()
        .unwrap()
        .on_item_status_changed(move |b, index| {
            println!("{}-{}", index, b);
            match SEND_TO_DART_MESSAGE_SINK.try_read() {
                Ok(s) => match s.as_ref() {
                    Some(s0) => {
                        let _b = Rust2DartResponse::<Request> {
                            data: Request {
                                title: format!("{}", index),
                                _type: 2,
                            },
                        };
                        s0.add(_b.to_json());
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

    pin_win_handle
        .upgrade()
        .unwrap()
        .set_todo_model(todo_model.into());

    pin_win_handle
}
