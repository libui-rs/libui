//! Demonstrates the use of the Window::save_file() call to get a filename via a friendly GUI,
//! and the Window::modal_err() call to display modal dialog boxes.

extern crate libui;
use libui::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let ui = UI::init().unwrap();

    // Menus must be created before the window. They belong to the window automatically.
    let menu_file = libui::menus::Menu::new("File");
    let menu_file_open = menu_file.append_item("Open");
    let menu_file_save = menu_file.append_item("Save");
    let menu_file_close = menu_file.append_item("Close");
    menu_file.append_separator();
    let menu_file_quit = menu_file.append_item("Exit");
    let menu_help = libui::menus::Menu::new("Help");
    let menu_help_about = menu_help.append_item("About");

    // Set up the application's layout
    let mut window = Window::new(&ui, "Text Editor", 640, 480, WindowType::HasMenubar);
    libui::build! { &ui,
        let layout = VerticalBox() {
            Stretchy: let entry = MultilineEntry()
        }
    }
    window.set_child(layout);

    menu_file_open.on_clicked({
        let mut entry = entry.clone();
        move |_, w| {
            let path = match w.open_file() {
                None => return,
                Some(v) => v,
            };

            let mut file = match File::open(&path) {
                Err(e) => {
                    w.modal_err(
                        "I/O Error",
                        &format!(
                            "Couldn't open file for reading {}: {}",
                            path.display(),
                            e.to_string()
                        ),
                    );
                    return;
                }
                Ok(f) => f,
            };

            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Err(_) => return,
                Ok(_) => (),
            };
            entry.set_value(&text);
        }
    });

    menu_file_save.on_clicked({
        let entry = entry.clone();
        move |_, w| {
            let path = match w.save_file() {
                None => return,
                Some(v) => v,
            };

            let mut file = match File::create(&path) {
                Err(e) => {
                    w.modal_err(
                        "I/O Error",
                        &format!(
                            "Couldn't open file for writing {}: {}",
                            path.display(),
                            e.to_string()
                        ),
                    );
                    return;
                }
                Ok(f) => f,
            };

            match file.write_all(entry.value().as_bytes()) {
                Err(_) => return,
                Ok(_) => (),
            };
        }
    });

    menu_file_close.on_clicked({
        let mut entry = entry.clone();
        move |_, _| {
            entry.set_value("");
        }
    });

    menu_file_quit.on_clicked({
        let ui = ui.clone();
        move |_, _| {
            ui.quit();
        }
    });

    menu_help_about.on_clicked(move |_, w| w.modal_msg("About", "libui: Menu Example"));

    window.show();
    ui.main();
}
