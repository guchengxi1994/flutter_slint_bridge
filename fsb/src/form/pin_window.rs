slint::slint! {
import { VerticalBox , HorizontalBox,StandardButton,LineEdit, Button,ListView,CheckBox} from "std-widgets.slint";

export struct ListViewItem  {
    title: string,
    checked: bool,
}

component CustomListView inherits Window {
    in property <[ListViewItem]> todo-model: [
        { title: "Implement the .slint file", checked: true },
        { title: "Do the Rust part", checked: false },
        { title: "Make the C++ code", checked: false },
        { title: "Write some JavaScript code", checked: false },
        { title: "Test the application", checked: false },
        { title: "Ship to customer", checked: false },
        { title: "???", checked: false },
        { title: "Profit", checked: false },
    ];

    in property <bool> show-header: false;
    in-out property <bool> is-sort-by-name: false;
    in-out property <bool> hide-done-items: false;

    callback todo-added(string);
    callback remove-done();
    callback popup_confirmed;
    callback show_confirm_popup;
    callback apply_sorting_and_filtering();

    show_confirm_popup => { confirm_popup.show(); }

    preferred-width: 400px;
    preferred-height: 600px;

    confirm_popup := PopupWindow {
        x: 40px;
        y: 100px;
        width: min(confirm_popup_layout.preferred-width, root.width - 80px);

        Rectangle {
            background: root.background;
            border-color: confirm_popup_text.color;
            border-width: 1px;
        }

        confirm_popup_layout := Dialog {
            height:100%; width: 100%;

            confirm_popup_text := Text {
                text: "Some items are not done, are you sure you wish to quit?";
                wrap: word-wrap;
            }

            StandardButton { kind: yes; clicked => { root.popup_confirmed(); } }
            StandardButton { kind: no; }
        }
    }


    VerticalBox {
        HorizontalBox {
            padding: 0px;
            text-edit := LineEdit {
                accepted(text) => {
                    root.todo-added(self.text);
                    self.text = "";
                }

                placeholder-text: "What needs to be done?";
            }

            btn := Button {
                clicked => {
                    root.todo-added(text-edit.text);
                    text-edit.text = "";
                }

                text: "Add New Entry";
                enabled: text-edit.text != "";
            }
        }

        if (root.show-header) : HorizontalBox {
            padding: 0px;
            alignment: start;

            CheckBox {
                toggled => {
                    root.apply_sorting_and_filtering();
                }

                text: "Sort by name";
                checked <=> root.is-sort-by-name;
            }

            CheckBox {
                toggled => {
                    root.apply_sorting_and_filtering();
                }

                text: "Hide done items";
                checked <=> root.hide-done-items;
            }
        }

        list-view := ListView {
            for todo in root.todo-model:  HorizontalLayout {
                CheckBox {
                    toggled => {
                        todo.checked = self.checked;
                    }

                    text: todo.title;
                    checked: todo.checked;
                }
            }
        }
        HorizontalBox {
            padding: 0px;
            alignment: end;

            Button {
                clicked => { root.remove-done(); }

                text: "Remove Done Items";
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
            Image {source: @image-url("tool.png") ;
                            width: 60px;
                            height: 50px;visible: !root.active;}
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