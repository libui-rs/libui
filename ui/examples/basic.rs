extern crate ui;

use ui::controls::{Button, Group, Label, VerticalBox, Form};
use ui::prelude::*;

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");

    // Create a window into which controls can be placed
    let mut win = Window::new(&ui.clone(), "Test App", 200, 200, WindowType::NoMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new();
    vbox.set_padded(true);

    let mut group_vbox = VerticalBox::new();
    let mut group = Group::new("Group");

    // Create two buttons to place in the window
    let mut button = Button::new("Button");
    button.on_clicked({
        move |btn| {
            btn.set_text("Clicked!");
        }
    });

    let mut quit_button = Button::new("Quit");


    let mut form = Form::new();
    form.set_padded(true);
    form.append("btn", quit_button.clone(), LayoutStrategy::Compact);
    quit_button.on_clicked({
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });
    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("There is a ton of text in this label.\n");
    label_text.push_str("Pretty much every unicode character is supported.\n");
    label_text.push_str("🎉 用户界面 사용자 인터페이스");
    let label = Label::new(&label_text);

    vbox.append(label, LayoutStrategy::Stretchy);
    group_vbox.append(button, LayoutStrategy::Compact);
    group_vbox.append(form, LayoutStrategy::Compact);
    group.set_child(group_vbox);
    vbox.append(group, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(vbox);
    // Show the window
    win.show();
    // Run the application
    ui.main();
}
