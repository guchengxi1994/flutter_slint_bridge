use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage {
    LabelMessage(String),
}

slint::slint! {
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component Dialog inherits Window {
        title: "对话框";
        width: 300px;
        height: 200px;
        no-frame: true;
        icon: @image-url("icon.png");

        Rectangle {
            background: @linear-gradient (90deg, #cde4ee 0%, #ebf8e1 100%);
        }

        callback close-dialog();

        VerticalBox {
            Text {
                text: "对话框";
                color: green;
            }
            Button {
                text: "返回数据并关闭";
                width: 120px;
                height: 40px;
                clicked => {
                    close-dialog()
                }
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::tests::Dialog;
    use slint::{ComponentHandle, PhysicalPosition};
    use tao::event_loop::ControlFlow;

    #[test]
    fn test2() -> anyhow::Result<()> {
        slint::slint! {
            export component Example inherits Window {
                preferred-width: 100px;
                preferred-height: 100px;
                Rectangle {
                    background: @linear-gradient (90deg, #3f87a6 0%, #ebf8e1 50%, #f69d3c 100%);
                }
            }
        }

        let window = Example::new()?;
        window.show()?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        // Do some work...
        window.hide()?; // The window will be auto disposed if no other references.
        anyhow::Ok(())
    }

    #[test]
    fn auto_hide_window() -> anyhow::Result<()> {
        let dialog = Dialog::new()?;
        dialog
            .window()
            .set_position(slint::PhysicalPosition::new(0, 0));
        let _dialog_handle = dialog.as_weak();
        dialog.on_close_dialog(move || {
            println!("close");
        });

        slint::Timer::single_shot(std::time::Duration::from_secs(2), move || {
            let _ = _dialog_handle.upgrade().unwrap().hide();
        });

        dialog.run()?;
        // dialog.show()?;
        // std::thread::spawn(move || {
        //     std::thread::sleep(std::time::Duration::from_secs(2));
        //     slint::invoke_from_event_loop(move || {
        //         let _ = _dialog_handle.unwrap().hide();
        //     })
        //     .unwrap();
        // });

        // slint::run_event_loop()?;

        anyhow::Ok(())
    }

    #[derive(Debug)]
    enum MyEvent2 {
        CustomEvent(String),
    }

    #[test]
    fn event_test() {
        let mut builder: tao::event_loop::EventLoopBuilder<MyEvent2> =
            tao::event_loop::EventLoopBuilder::<MyEvent2>::with_user_event();
        tao::platform::windows::EventLoopBuilderExtWindows::with_any_thread(&mut builder, true);

        let event_loop: tao::event_loop::EventLoop<MyEvent2> = builder.build();

        let proxy = event_loop.create_proxy();

        // 在另一个线程中发送一个自定义事件
        std::thread::spawn(move || {
            proxy
                .send_event(MyEvent2::CustomEvent("Hello from another thread".into()))
                .unwrap();
        });

        event_loop.run(move |event, _, _control_flow| match event {
            tao::event::Event::UserEvent(my_event) => match my_event {
                MyEvent2::CustomEvent(message) => {
                    println!("Received custom event: {:?}", message);
                    *_control_flow = ControlFlow::Exit;
                }
            },
            e => {
                println!("Unsupport event: {:?}", e);
            }
        });
    }

    #[test]
    fn auto_close_dialog_test() {
        crate::dialog::show_dialog(None);
    }

    #[test]
    fn test_event_loop() {
        let _ = crate::event_loop::create_event_loop();
    }

    // #[test]
    // fn test_drag_window()-> anyhow::Result<()>{
    //     let dialog = crate::form::draggable_notification::HelloWorld2::new()?;
    //     dialog.run()?;
    //     anyhow::Ok(())
    // }

    #[test]
    fn test_drag_window2() -> anyhow::Result<()> {
        let pin_win = crate::form::pin_window::PinWindow::new()?;
        pin_win
            .window()
            .set_position(slint::LogicalPosition::new(0., 0.));

        let pin_win_clone = pin_win.as_weak();
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

        pin_win.run()?;
        anyhow::Ok(())
    }

    #[test]
    fn test_drag_window3() -> anyhow::Result<()> {
        let mut pin_win = crate::form::pin_window::PinWindow::new()?;

        pin_win = crate::event_loop::pin_window_wrapper::pin_window_wrapper(pin_win);

        pin_win.run()?;
        anyhow::Ok(())
    }
}
