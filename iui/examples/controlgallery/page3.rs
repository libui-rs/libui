
extern crate iui;
use iui::controls::*;
use iui::prelude::*;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

pub fn make_data_page(ui: UI, window: Rc<RefCell<Window>>) -> Control {

    // Left
    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);

    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let bt_color = ColorButton::new(&ui);
    let bt_date = DateTimePicker::new(&ui, DateTimePickerKind::Date);
    let bt_time = DateTimePicker::new(&ui, DateTimePickerKind::Time);
    let bt_datetime = DateTimePicker::new(&ui, DateTimePickerKind::DateTime);
    let bt_font = FontButton::new(&ui);
    let mut bt_test = Button::new(&ui, "Test");
    bt_test.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut bt_color = bt_color.clone();
        let bt_datetime = bt_datetime.clone();
        let bt_font = bt_font.clone();
        move |_| -> () {
            let mut c = bt_color.color(&ui);
            c.1 = 1.0;
            bt_color.set_color(&ui, c.0, c.1, c.2, c.3);

            let mut time = bt_datetime.datetime(&ui);
            time .tm_hour = 0;
            bt_datetime.set_datetime(&ui, time);

            let font = bt_font.font(&ui);
            window.borrow().modal_msg(&ui, "Font: ", format!("{font:?}").as_str());
        }
    });

    // TODO: DatePicker
    // TODO: TimePicker
    // TODO: DateTimePicker
    // TODO: FontPicker
    // TODO: ColorPicker
    // TODO: Vertical Separator
    vbox.append(&ui, bt_date, LayoutStrategy::Compact);
    vbox.append(&ui, bt_time, LayoutStrategy::Compact);
    vbox.append(&ui, bt_datetime, LayoutStrategy::Compact);
    vbox.append(&ui, bt_color, LayoutStrategy::Compact);
    vbox.append(&ui, bt_font, LayoutStrategy::Compact);
    vbox.append(&ui, bt_test, LayoutStrategy::Compact);
    hbox.append(&ui, vbox, LayoutStrategy::Stretchy);

    let mut grid = LayoutGrid::new(&ui);
    grid.set_padded(&ui, true);

    // Column 1 - Output boxes
    let (tb_open_file, tb_open_folder, tb_save_file) = {
        let tb_open_file = Entry::new(&ui);
        let tb_open_folder = Entry::new(&ui);
        let tb_save_file = Entry::new(&ui);


        grid.append(
            &ui,
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
            &ui,
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
            &ui,
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
        let bt_open_file = Button::new(&ui, "Open File");
        let bt_open_folder_pwd = Button::new(&ui, "Open Folder");
        let bt_save_file = Button::new(&ui, "Save File");


        grid.append(
            &ui,
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
            &ui,
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
            &ui,
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

    bt_open_file.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut tb_open_file = tb_open_file.clone();
        move |_| -> () {
            match window.borrow_mut().open_file(&ui) {
                Some(path) => tb_open_file.set_value(&ui, path.to_str().unwrap()),
                None => (),
            }
        }
    });

    bt_open_folder_pwd.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut tb_open_folder = tb_open_folder.clone();
        move |_| -> () {
            match window.borrow_mut().open_folder(&ui) {
                Some(path) => tb_open_folder.set_value(&ui, path.to_str().unwrap()),
                None => (),
            }      
        }
    });

    bt_save_file.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut tb_save_file = tb_save_file.clone();
        move |_| -> () {
            match window.borrow_mut().save_file(&ui) {
                Some(path) => tb_save_file.set_value(&ui, path.to_str().unwrap()),
                None => (),
            }
        }
    });

    let mut bt_msgbox = Button::new(&ui, "Message Box");
    bt_msgbox.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_msg(&ui, "This is a normal message box.", "More detailed information can be shown here.");
        }
    });

    let mut bt_msgbox_error = Button::new(&ui, "Error Message");
    bt_msgbox_error.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        move |_| -> () {
            window.borrow_mut().modal_err(&ui, "This message box describes an error.", "More detailed information can be shown here.");
        }
    });

    grid.append(
        &ui,
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
        &ui,
        bt_msgbox_error.clone(),
        0,
        4,
        1,
        1,
        GridExpand::Vertical,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    let mut vbox2 = VerticalBox::new(&ui);
    vbox2.set_padded(&ui, true);
    vbox2.append(&ui, grid, LayoutStrategy::Compact);
    hbox.append(&ui, vbox2, LayoutStrategy::Stretchy);

    return hbox.into();
}