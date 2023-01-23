
extern crate iui;
use iui::controls::*;
use iui::prelude::*;

pub fn make_numbers_page(ui: UI,) -> Control {

    // Left
    let mut hbox = HorizontalBox::new(&ui);
    let mut group_numbers = Group::new(&ui, "Numbers");
    let mut vbox_numbers = VerticalBox::new(&ui);
    vbox_numbers.set_padded(&ui, true);

    let mut spinner = Spinbox::new(&ui, 0, 100);
    let mut slider = Slider::new(&ui, 0, 100);
    let progressbar = ProgressBar::new();

    spinner.on_changed(&ui, {
        let ui = ui.clone();
        let mut slider = slider.clone();
        let mut progressbar = progressbar.clone();
        move |val| -> () {
            slider.set_value(&ui, val);
            progressbar.set_value(&ui, val as u32);
        }
    });

    slider.on_changed(&ui, {
        let ui = ui.clone();
        let mut spinner = spinner.clone();
        let mut progressbar = progressbar.clone();
        move |val| -> () {
            spinner.set_value(&ui, val);
            progressbar.set_value(&ui, val as u32);
        }
    });

    vbox_numbers.append(&ui, spinner, LayoutStrategy::Compact);
    vbox_numbers.append(&ui, slider, LayoutStrategy::Compact);
    vbox_numbers.append(&ui, progressbar, LayoutStrategy::Compact);

    group_numbers.set_child(&ui, vbox_numbers);
    hbox.append(&ui, group_numbers.clone(), LayoutStrategy::Stretchy);


    // Right
    let mut group_lists = Group::new(&ui, "Lists");
    let mut vbox_lists = VerticalBox::new(&ui);
    vbox_lists.set_padded(&ui, true);

    let combobox = Combobox::new(&ui);
    combobox.append(&ui, "Combobox Item 1");
    combobox.append(&ui, "Combobox Item 2");
    combobox.append(&ui, "Combobox Item 3");

    let mut combobox_editable = EditableCombobox::new(&ui);
    combobox_editable.append(&ui, "Editable Item 1");
    combobox_editable.append(&ui, "Editable Item 2");
    combobox_editable.append(&ui, "Editable Item 3");
    combobox_editable.set_value(&ui, "Custom Text");

    let radiobuttons = RadioButtons::new(&ui);
    radiobuttons.append(&ui, "Radio Button 1");
    radiobuttons.append(&ui, "Radio Button 2");
    radiobuttons.append(&ui, "Radio Button 3");

    vbox_lists.append(&ui, combobox, LayoutStrategy::Compact);
    vbox_lists.append(&ui, combobox_editable, LayoutStrategy::Compact);
    vbox_lists.append(&ui, radiobuttons, LayoutStrategy::Compact);

    group_lists.set_child(&ui, vbox_lists);
    hbox.append(&ui, group_lists.clone(), LayoutStrategy::Stretchy);

    return hbox.into();
}