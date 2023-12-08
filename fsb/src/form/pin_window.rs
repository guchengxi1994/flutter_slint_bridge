slint::slint! {
import { VerticalBox , HorizontalBox, Button,ListView,CheckBox} from "std-widgets.slint";

export component CustomListView inherits Window {
    ListView {
        width: 300px;
        height: 500px;
        for data in [
            { text: "Blue", color: #0000ff, bg: #eeeeee},
            { text: "Red", color: #ff0000, bg: #eeeeee},
            { text: "Green", color: #00ff00, bg: #eeeeee},
            { text: "Yellow", color: #ffff00, bg: #222222 },
            { text: "Black", color: #000000, bg: #eeeeee },
            { text: "White", color: #ffffff, bg: #222222 },
            { text: "Magenta", color: #ff00ff, bg: #eeeeee },
            { text: "Cyan", color: #00ffff, bg: #222222 },
        ] : Rectangle {
            height: 30px;
            // background: data.bg;
            width: parent.width;
            CheckBox {}
            Text {
                x: 0;
                text: data.text;
                // color: data.color;
            }
        }
    }
}

export component PinWindow inherits Window {
        property <bool> active: false;
        no-frame: true;
        always-on-top: true;
        background: transparent;
        property <length> dynamic_width: 100px;
        property <length> dynamic_height: 100px;
        property <length> radius: 100px;
        in property <string> title_name : "Default";


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
                if (!root.active){
                    root.mouse_move(self.mouse-x - self.pressed-x, self.mouse-y - self.pressed-y);
                }
            }
            clicked => {
                if (!root.active){
                    root.active = !root.active;
                }
                // root.active = !root.active;
            }

            VerticalLayout {
                width: dynamic_width;
                height: dynamic_height;
                visible: root.active;
                alignment: center;


                Rectangle {
                    height: 30px;
                    TouchArea {
                        moved => {
                            if (root.active){
                                root.mouse_move(self.mouse-x - self.pressed-x, self.mouse-y - self.pressed-y);
                            }
                        }
                        HorizontalLayout {
                        alignment: space-around;
                        Image {
                            source: @image-url("icon.png") ;
                            width: 30px;
                            height: 30px;
                        }
                        Rectangle {
                            width: dynamic_width -70px ;
                            height: 30px;
                            Text {
                                text: title_name;
                            }
                        }
                        TouchArea {
                            width: 30px;
                            height: 30px;
                            clicked => {
                                root.active = !root.active;
                            }
                            Image {source: @image-url("hide.png") ;
                            width: 20px;
                            height: 20px;}
                        }
                    }
                    }   
                }
                Rectangle {
                    height: dynamic_height - 30px;
                    width: dynamic_width;
                    VerticalLayout {
                        alignment: start;
                        HorizontalLayout {
                            alignment: center;
                            CustomListView{}
                        }
                        
                    }
                    
                }
            }

        }
    }

}
