use std::rc::Rc;

use crate::ui::ListViewItem;
use crate::ui::PinWindow;
use slint::ComponentHandle;
use slint::Model;

use super::{model::Rust2DartResponse, PIN_WINDOW_ITEMS, SEND_TO_DART_MESSAGE_SINK};

#[allow(unused_assignments)]
pub fn pin_window_wrapper(pin_win: PinWindow) -> PinWindow {
    let pin_win_clone = pin_win.as_weak();

    let inited_items;

    {
        inited_items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
    }

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
        let todo_model = todo_model.clone();
        let pin_win_clone = pin_win_clone.unwrap();
        move || {
            // handle.upgrade().unwrap().set_todo_model(todo_model.clone().into())
            // TODO 告诉dart重新同步
            let items;

            {
                items = (*(PIN_WINDOW_ITEMS.read().unwrap())).clone();
            }
            println!("length : {:?}", items.len());
            loop {
                if todo_model.row_count() == 0 {
                    break;
                }
                todo_model.remove(0);
            }
            // todo_model = Rc::new(slint::VecModel::<ListViewItem>::default());
            for i in items {
                todo_model.push(ListViewItem {
                    checked: i.checked,
                    title: i.title.into(),
                    id: i.id,
                });
            }
            for i in 0..todo_model.row_count() {
                println!("{:?}", todo_model.row_data(i));
            }

            pin_win_clone.set_todo_model(todo_model.clone().into());
        }
    });

    pin_win.on_mouse_move(move |delta_x, delta_y| {
        let pin_win_clone = pin_win_clone.unwrap();
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
    });

    pin_win.on_todo_added({
        let todo_model = todo_model.clone();

        move |str| {
            match SEND_TO_DART_MESSAGE_SINK.try_read() {
                Ok(s) => match s.as_ref() {
                    Some(s0) => {
                        let b = Rust2DartResponse::<String> {
                            data: str.to_string(),
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

            todo_model.push(ListViewItem {
                checked: false,
                title: str,
                id: 1,
            });
            for i in 0..todo_model.row_count() {
                println!("{:?}", todo_model.row_data(i));
            }

            // 然后把这个 item 塞到 PIN_WINDOW_ITEMS 中
        }
    });

    pin_win.on_item_status_changed(move |b, index| println!("{}-{}", index, b));

    pin_win.set_todo_model(todo_model.into());

    pin_win
}
