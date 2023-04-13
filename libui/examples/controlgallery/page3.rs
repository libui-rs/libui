use libui::controls::*;
use libui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn make_data_page(_ui: UI, window: Rc<RefCell<Window>>) -> Control {
    // Left
    let mut hbox = HorizontalBox::new();
    hbox.set_padded(true);

    let mut vbox = VerticalBox::new();
    vbox.set_padded(true);

    let bt_color = ColorButton::new();
    let bt_date = DateTimePicker::new(DateTimePickerKind::Date);
    let bt_time = DateTimePicker::new(DateTimePickerKind::Time);
    let bt_datetime = DateTimePicker::new(DateTimePickerKind::DateTime);
    let bt_font = FontButton::new();
    let mut bt_test = Button::new("Test");
    bt_test.on_clicked({
        let window = window.clone();
        let mut bt_color = bt_color.clone();
        let bt_datetime = bt_datetime.clone();
        let bt_font = bt_font.clone();
        move |_| -> () {
            let mut c = bt_color.color();
            c.1 = 1.0;
            bt_color.set_color(c.0, c.1, c.2, c.3);

            let mut time = bt_datetime.datetime();
            time.tm_hour = 0;
            bt_datetime.set_datetime(time);

            let font = bt_font.font();
            window
                .borrow()
                .modal_msg("Font: ", format!("{font:?}").as_str());
        }
    });

    // TODO: Vertical Separator
    vbox.append(bt_date, LayoutStrategy::Compact);
    vbox.append(bt_time, LayoutStrategy::Compact);
    vbox.append(bt_datetime, LayoutStrategy::Compact);
    vbox.append(bt_color, LayoutStrategy::Compact);
    vbox.append(bt_font, LayoutStrategy::Compact);
    vbox.append(bt_test, LayoutStrategy::Compact);
    hbox.append(vbox, LayoutStrategy::Stretchy);

    let mut grid = LayoutGrid::new();
    grid.set_padded(true);

    // Column 1 - Output boxes
    let (tb_open_file, tb_open_folder, tb_save_file) = {
        let tb_open_file = Entry::new();
        let tb_open_folder = Entry::new();
        let tb_save_file = Entry::new();

        grid.append(
            tb_open_file.clone(),
            1,
            0,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            tb_open_folder.clone(),
            1,
            1,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            tb_save_file.clone(),
            1,
            2,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        (tb_open_file, tb_open_folder, tb_save_file)
    };

    // Column 0 - Label
    let (mut bt_open_file, mut bt_open_folder_pwd, mut bt_save_file) = {
        let bt_open_file = Button::new("Open File");
        let bt_open_folder_pwd = Button::new("Open Folder");
        let bt_save_file = Button::new("Save File");

        grid.append(
            bt_open_file.clone(),
            0,
            0,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            bt_open_folder_pwd.clone(),
            0,
            1,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            bt_save_file.clone(),
            0,
            2,
            1,
            1,
            GridExpand::Vertical,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        (bt_open_file, bt_open_folder_pwd, bt_save_file)
    };

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

    let mut bt_msgbox = Button::new("Message Box");
    bt_msgbox.on_clicked({
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_msg(
                "This is a normal message box.",
                "More detailed information can be shown here.",
            );
        }
    });

    let mut bt_msgbox_error = Button::new("Error Message");
    bt_msgbox_error.on_clicked({
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_err(
                "This message box describes an error.",
                "More detailed information can be shown here.",
            );
        }
    });

    grid.append(
        bt_msgbox.clone(),
        0,
        3,
        1,
        1,
        GridExpand::Vertical,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    grid.append(
        bt_msgbox_error.clone(),
        0,
        4,
        1,
        1,
        GridExpand::Vertical,
        GridAlignment::Fill,
        GridAlignment::Start, // Avoids inconsistencies on OSX
    );

    let mut vbox2 = VerticalBox::new();
    vbox2.set_padded(true);
    vbox2.append(grid, LayoutStrategy::Compact);
    hbox.append(vbox2, LayoutStrategy::Stretchy);

    return hbox.into();
}
