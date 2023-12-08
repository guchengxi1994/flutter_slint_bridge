slint::slint! {
    import { Button , HorizontalBox, VerticalBox} from "std-widgets.slint";
    export component Notification inherits Window {
        in property <string> window_title : "Default";

        title: root.window-title;
        width: 200px;
        height: 100px;
        no-frame: true;
        always-on-top: true;
        icon: @image-url("icon.png");

        in property <string> content : "Default";

        Rectangle {
            border-radius: 4px;
            background: @linear-gradient (90deg, #cde4ee 0%, #ebf8e1 100%);
        }

        VerticalBox {
            alignment: center;
            HorizontalBox {
                alignment: center;
                Text {
                    text: root.content;
                    font-size: 14px;
                    color: green;
                }
            }
        }
    }
}
