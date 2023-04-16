extern crate libui;

use libui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Initialize the UI framework
    let ui = UI::init().unwrap();

    // The macro based builder vastly reduces the amount of code required
    // to build complex user interfaces. Compare this example to the "controlgallery".
    // It creates mostly the same UI in a fraction of lines.
    libui::layout! { &ui,
        let layout = HorizontalBox() {
            Stretchy: let tabs = TabGroup() {
                // TAB PAGE 1
                ("Basic Controls", margined: false): let page1 = VerticalBox() {
                    Compact: let hb = HorizontalBox() {
                        Compact: let btn = Button("Button")
                        Compact: let btn = Checkbox("Checkbox")
                    }
                    Compact: let lb = Label("This is a label.\nLabels can span multiple lines.")
                    Compact: let _hs = HorizontalSeparator()
                    Stretchy: let input_group = Group("Entries") {
                        let form = Form(padded: true) {
                            (Compact, "Entry"): let entry = Entry()
                            (Compact, "Password Entry"): let entry_pwd = Entry()
                            (Compact, "Search Entry"): let entry_search = SearchEntry()
                            (Stretchy, "Multiline Entry"): let entry_multi = MultilineEntry()
                            (Stretchy, "Non-wrapping Entry"): let entry_nowrap = MultilineEntry( wrapping: false )
                        }
                    }
                }
                // TAB PAGE 2
                ("Numbers and Lists", margined: false): let page2 = HorizontalBox() {
                    Stretchy: let g1 = Group("Numbers") {
                        let _vb1 = VerticalBox(padded: true) {
                            Compact: let spinner = Spinbox(0, 100)
                            Compact: let slider = Slider(0, 100)
                            Compact: let progress = ProgressBar()
                        }
                    }
                    Stretchy: let g1 = Group("Lists") {
                        let _vb2 = VerticalBox(padded: true) {
                            Compact: let combo = Combobox(selected: 1) {
                                "Combobox Item 1", "Combobox Item 2", "Combobox Item 3"
                            }
                            Compact: let combobox_editable = EditableCombobox() {
                                "Editable Item 1", "Editable Item 2", "Editable Item 3"
                            }
                            Compact: let radiobuttons = RadioButtons(selected: 0) {
                                "Radio Button 1", "Radio Button 2", "Radio Button 3"
                            }
                        }
                    }
                }
                // TAB PAGE 3
                ("Data Choosers", margined: false): let page2 = HorizontalBox(padded: true) {
                    Stretchy: let _vb1 = VerticalBox(padded: true) {
                        Compact: let bt_date = DateTimePicker(Date)
                        Compact: let bt_time = DateTimePicker(Time)
                        Compact: let bt_datetime = DateTimePicker(DateTime)
                        Compact: let bt_color = ColorButton()
                        Compact: let bt_font = FontButton()
                    }
                    Stretchy: let _vb2 = VerticalBox(padded: true) {
                        Compact: let grid = LayoutGrid(padded: true) {
                            (0, 0)(1, 1) Vertical (Fill, Fill) : let bt_open_file = Button("Open File")
                            (0, 1)(1, 1) Vertical (Fill, Fill) : let bt_open_folder_pwd = Button("Open Folder")
                            (0, 2)(1, 1) Vertical (Fill, Fill) : let bt_save_file = Button("Save File")
                            (0, 3)(1, 1) Vertical (Fill, Fill) : let bt_msgbox = Button("Message Box")
                            (0, 4)(1, 1) Vertical (Fill, Fill) : let bt_msgbox_error = Button("Error Message")
                            (1, 0)(1, 1) Vertical (Fill, Fill) : let tb_open_file = Entry()
                            (1, 1)(1, 1) Vertical (Fill, Fill) : let tb_open_folder = Entry()
                            (1, 2)(1, 1) Vertical (Fill, Fill) : let tb_save_file = Entry()
                        }
                    }
                }
            }
        }
    }

    // Display the layout in a window
    let window = Rc::new(RefCell::new(Window::new(
        &ui,
        "Control Gallery (Builder Macro)",
        640,
        480,
        WindowType::NoMenubar,
    )));
    window.borrow_mut().set_child(layout);

    // You can easily reference the controls declared in the `libui::build!` macro.
    // Note that all controls are created mutable, contrary to what the macro syntax
    // suggests. This is because most control will be mutated in user code anyways.
    bt_open_file.on_clicked({
        let window = window.clone();
        let mut tb_open_file = tb_open_file.clone();
        move |_| -> () {
            match window.borrow_mut().open_file() {
                Some(path) => tb_open_file.set_value(path.to_str().unwrap()),
                None => (),
            }
        }
    });

    bt_open_folder_pwd.on_clicked({
        let window = window.clone();
        let mut tb_open_folder = tb_open_folder.clone();
        move |_| -> () {
            match window.borrow_mut().open_folder() {
                Some(path) => tb_open_folder.set_value(path.to_str().unwrap()),
                None => (),
            }
        }
    });

    bt_save_file.on_clicked({
        let window = window.clone();
        let mut tb_save_file = tb_save_file.clone();
        move |_| -> () {
            match window.borrow_mut().save_file() {
                Some(path) => tb_save_file.set_value(path.to_str().unwrap()),
                None => (),
            }
        }
    });

    bt_msgbox.on_clicked({
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_msg(
                "This is a normal message box.",
                "More detailed information can be shown here.",
            );
        }
    });

    bt_msgbox_error.on_clicked({
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_err(
                "This message box describes an error.",
                "More detailed information can be shown here.",
            );
        }
    });

    // Display and run
    window.borrow_mut().show();
    ui.main();
}
