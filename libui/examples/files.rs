//! Demonstrates the use of the Window::save_file() call to get a filename via a friendly GUI,
//! and the Window::modal_err() call to display modal dialog boxes.

extern crate libui;
use libui::controls::{Button, MultilineEntry, VerticalBox};
use libui::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Initialize the UI
    let ui = UI::init().unwrap();

    // Create the input controls
    let entry = MultilineEntry::new();
    let mut button = Button::new("Save Buffer");

    // Set up the application's layout
    let mut window = Window::new(&ui, "Save Buffer to File", 640, 480, WindowType::NoMenubar);
    let mut vbox = VerticalBox::new();
    vbox.append(entry.clone(), LayoutStrategy::Stretchy);
    vbox.append(button.clone(), LayoutStrategy::Compact);
    window.set_child(vbox);
    window.show();

    // When the button is clicked, get the name of a file and then write the entry's contents to it.
    // Note the in real code you should spin off a thread to do the actual writing, do it between UI events,
    // or use Tokio. Even with minmal content, this method shows noticable lag.
    button.on_clicked({
        move |_| {
            if let Some(path) = window.save_file() {
                let mut file = match File::create(&path) {
                    Err(why) => {
                        window.modal_err(
                            "I/O Error",
                            &format!(
                                "Could not open file {}: {}",
                                path.display(),
                                why.to_string()
                            ),
                        );
                        return;
                    }
                    Ok(f) => f,
                };
                match file.write_all(entry.value().as_bytes()) {
                    Err(why) => {
                        window.modal_err(
                            "I/O Error",
                            &format!(
                                "Could not write to file {}: {}",
                                path.display(),
                                why.to_string()
                            ),
                        );
                        return;
                    }
                    Ok(_) => (),
                };
            }
        }
    });

    ui.main();
}
