//! Demonstrates a mutable application state manipulated over a number of UIs

extern crate libui;
use libui::controls::{
    Entry, Group, HorizontalBox, HorizontalSeparator, Label, MultilineEntry, PasswordEntry,
    ProgressBar, Slider, Spacer, Spinbox, VerticalBox,
};
use libui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// This struct will hold the values that multiple callbacks will need to access.
struct State {
    slider_val: i32,
    spinner_val: i32,
    entry_val: String,
    password_val: String,
    multi_val: String,
}

fn main() {
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    // Initialize the state of the application.
    let state = Rc::new(RefCell::new(State {
        slider_val: 1,
        spinner_val: 1,
        entry_val: "".into(),
        password_val: "".into(),
        multi_val: "".into(),
    }));

    // Set up the inputs for the application.
    // While it's not necessary to create a block for this, it makes the code a lot easier
    // to read; the indentation presents a visual cue informing the reader that these
    // statements are related.
    let (input_group, mut slider, mut spinner, mut entry, mut password, mut multi) = {
        // The group will hold all the inputs
        let mut input_group = Group::new("Inputs");
        // The vertical box arranges the inputs within the groups
        let mut input_vbox = VerticalBox::new();
        input_vbox.set_padded(true);
        // Numerical inputs
        let slider = Slider::new(1, 100);
        let spinner = Spinbox::new(1, 100);
        let entry = Entry::new();
        let password = PasswordEntry::new();
        let multi = MultilineEntry::new();
        // Add everything in hierarchy
        // Note the reverse order here. Again, it's not necessary, but it improves
        // readability.
        input_vbox.append(slider.clone(), LayoutStrategy::Compact);
        input_vbox.append(spinner.clone(), LayoutStrategy::Compact);
        input_vbox.append(Spacer::new(), LayoutStrategy::Compact);
        input_vbox.append(HorizontalSeparator::new(), LayoutStrategy::Compact);
        input_vbox.append(Spacer::new(), LayoutStrategy::Compact);
        input_vbox.append(entry.clone(), LayoutStrategy::Compact);
        input_vbox.append(password.clone(), LayoutStrategy::Compact);
        input_vbox.append(multi.clone(), LayoutStrategy::Stretchy);
        input_group.set_child(input_vbox);
        (input_group, slider, spinner, entry, password, multi)
    };

    // Set up the outputs for the application. Organization is very similar to the
    // previous setup.
    let (
        output_group,
        add_label,
        sub_label,
        text_label,
        password_label,
        bigtext_label,
        progress_bar,
    ) = {
        let mut output_group = Group::new("Outputs");
        let mut output_vbox = VerticalBox::new();
        let add_label = Label::new("");
        let sub_label = Label::new("");
        let text_label = Label::new("");
        let password_label = Label::new("");
        let bigtext_label = Label::new("");
        let progress_bar = ProgressBar::indeterminate();
        output_vbox.append(add_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(sub_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(progress_bar.clone(), LayoutStrategy::Compact);
        output_vbox.append(text_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(password_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(bigtext_label.clone(), LayoutStrategy::Stretchy);
        output_group.set_child(output_vbox);
        (
            output_group,
            add_label,
            sub_label,
            text_label,
            password_label,
            bigtext_label,
            progress_bar,
        )
    };

    // This horizontal box will arrange the two groups of controls.
    let mut hbox = HorizontalBox::new();
    hbox.append(input_group, LayoutStrategy::Stretchy);
    hbox.append(output_group, LayoutStrategy::Stretchy);

    // The window allows all constituent components to be displayed.
    let mut window = Window::new(
        &ui.clone(),
        "Input Output Test",
        300,
        150,
        WindowType::NoMenubar,
    );
    window.set_child(hbox);
    window.show();

    // These on_changed functions allow updating the application state when a
    // control changes its value.

    slider.on_changed({
        let state = state.clone();
        move |val| {
            state.borrow_mut().slider_val = val;
        }
    });

    spinner.on_changed({
        let state = state.clone();
        move |val| {
            state.borrow_mut().spinner_val = val;
        }
    });

    entry.on_changed({
        let state = state.clone();
        move |val| {
            state.borrow_mut().entry_val = val;
        }
    });

    password.on_changed({
        let state = state.clone();
        move |val| {
            state.borrow_mut().password_val = val;
        }
    });

    multi.on_changed({
        let state = state.clone();
        move |val| {
            state.borrow_mut().multi_val = val;
        }
    });

    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick({
        let mut add_label = add_label.clone();
        let mut sub_label = sub_label.clone();
        let mut text_label = text_label.clone();
        let mut password_label = password_label.clone();
        let mut bigtext_label = bigtext_label.clone();
        let mut progress_bar = progress_bar.clone();
        move || {
            let state = state.borrow();

            // Update all the outputs
            add_label.set_text(&format!("Added: {}", state.slider_val + state.spinner_val));
            sub_label.set_text(&format!(
                "Subtracted: {}",
                state.slider_val - state.spinner_val
            ));
            text_label.set_text(&format!("Text: {}", state.entry_val));
            password_label.set_text(&format!("Secret Text: {}", state.password_val));
            bigtext_label.set_text(&format!("Multiline Text: {}", state.multi_val));
            progress_bar.set_value((state.slider_val + state.spinner_val) as u32)
        }
    });
    event_loop.run();
}
