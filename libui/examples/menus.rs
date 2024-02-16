//! Demonstrates the use of the Window::save_file() call to get a filename via a friendly GUI,
//! and the Window::modal_err() call to display modal dialog boxes.

extern crate libui;
use libui::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let ui = UI::init().unwrap();

    // Menus must be created before the window. They belong to the window automatically.
    libui::menu! { &ui,
        let menu_file = Menu("File") {
            let menu_file_open = MenuItem("Open")
            let menu_file_save = MenuItem("Save")
            Separator()
            let menu_file_close = MenuItem("Close")
            let menu_file_pref = PreferencesItem()
            let _menu_file_quit = QuitItem()
        }
        let menu_help = Menu("Help") {
            let _menu_help_updates = MenuItem("Updates")
            let menu_help_about = AboutItem()
        }
    }

    // Set up the application's layout
    let mut window = Window::new(&ui, "Text Editor", 640, 480, WindowType::HasMenubar);
    libui::layout! { &ui,
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

    menu_file_pref
        .on_clicked(move |_, w| w.modal_msg("Preferences", "Preferences menu item clicked!"));

    menu_help_about.on_clicked(move |_, w| w.modal_msg("About", "libui: Menu Example"));

    // The special MenuItem from `Menu::append_quit_item()` or a `libui::menu! { QuitMenuItem() }` macro
    // doesn't accept a callback with MenuItem::on_clicked(). Instead, call UI::on_should_quit() instead.
    ui.on_should_quit({
        let ui = ui.clone();
        move || {
            ui.quit();
        }
    });

    window.show();
    ui.main();
}
