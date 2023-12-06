slint::slint! {
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component ConfirmDialog inherits Window {
        in property <string> window_title : "Default";
        in property <length> window_width : 200px;
        in property <length> window_height : 100px;

        title: root.window-title;
        no-frame: false;
        width: window_width;
        height: window_height;
        icon: @image-url("icon.png");

        in property <string> content : "Default";

        // Rectangle {
        //     border-radius: 4px;
        //     background: @linear-gradient (90deg, #cde4ee 0%, #ebf8e1 100%);
        // }

        callback close-dialog();

        VerticalBox {
            alignment: start;
            HorizontalBox {
                alignment: start;
                Text {
                    text: root.content;
                    font-size: 14px;
                    color: green;
                }
            }
            HorizontalBox {
                alignment: end;
                Button {
                    text: "确定";
                    width: 60px;
                    height: 30px;
                clicked => {
                    close-dialog()
                    }
                }
            }
        }
    }
}
