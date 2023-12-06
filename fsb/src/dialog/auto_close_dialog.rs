use super::PROXY;

slint::slint! {
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component AutoCloseDialog inherits Window {
        // title: "对话框";
        width: 300px;
        height: 200px;
        no-frame: true;
        icon: @image-url("icon.png");

        Rectangle {
            background: @linear-gradient (90deg, #cde4ee 0%, #ebf8e1 100%);
        }

        callback close-dialog();

        // VerticalBox {
        //     Text {
        //         text: "对话框";
        //         color: green;
        //     }
        //     Button {
        //         text: "返回数据并关闭";
        //         width: 120px;
        //         height: 40px;
        //         clicked => {
        //             close-dialog()
        //         }
        //     }
        // }
    }
}

pub fn show_auto_close_dialog() {
    let r = PROXY.read().unwrap();
    let _ = r.clone().unwrap().send_event(super::MyEvent::CustomEvent(
        "Hello from another thread".into(),
    ));
}
