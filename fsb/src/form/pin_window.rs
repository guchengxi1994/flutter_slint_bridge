slint::slint! {
export component PinWindow inherits Window {
        property <bool> active: false;
        no-frame: true;
        always-on-top: true;
        background: transparent;
        property <length> dynamic_width: 100px;
        property <length> dynamic_height: 100px;
        property <length> radius: 100px;


        callback mouse_move(length, length);

        Rectangle {
            border-radius: radius;
            width: dynamic_width;
            height: dynamic_height;
            background: @linear-gradient (90deg, #cde4ee 0%, #ebf8e1 100%);
        }

        states [
            inactive when !active:{
                dynamic_width:100px;
                dynamic_height:100px;
                radius:100px;
            }
            active when active:{
                dynamic_width:300px;
                dynamic_height:600px;
                radius:4px;
            }
        ]

        TouchArea {
            moved => {
                root.mouse_move(self.mouse-x - self.pressed-x, self.mouse-y - self.pressed-y);
            }
            clicked => {
                root.active = !root.active;
            }
        }
    }

}
