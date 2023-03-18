use ui::controls::*;
use ui::prelude::*;

pub fn make_numbers_page(_ui: UI) -> Control {
    // Left
    let mut hbox = HorizontalBox::new();
    let mut group_numbers = Group::new("Numbers");
    let mut vbox_numbers = VerticalBox::new();
    vbox_numbers.set_padded(true);

    let mut spinner = Spinbox::new(0, 100);
    let mut slider = Slider::new(0, 100);
    let progressbar = ProgressBar::new();

    spinner.on_changed({
        let mut slider = slider.clone();
        let mut progressbar = progressbar.clone();
        move |val| -> () {
            slider.set_value(val);
            progressbar.set_value(val as u32);
        }
    });

    slider.on_changed({
        let mut spinner = spinner.clone();
        let mut progressbar = progressbar.clone();
        move |val| -> () {
            spinner.set_value(val);
            progressbar.set_value(val as u32);
        }
    });

    vbox_numbers.append(spinner, LayoutStrategy::Compact);
    vbox_numbers.append(slider, LayoutStrategy::Compact);
    vbox_numbers.append(progressbar, LayoutStrategy::Compact);

    group_numbers.set_child(vbox_numbers);
    hbox.append(group_numbers.clone(), LayoutStrategy::Stretchy);

    // Right
    let mut group_lists = Group::new("Lists");
    let mut vbox_lists = VerticalBox::new();
    vbox_lists.set_padded(true);

    let combobox = Combobox::new();
    combobox.append("Combobox Item 1");
    combobox.append("Combobox Item 2");
    combobox.append("Combobox Item 3");

    let mut combobox_editable = EditableCombobox::new();
    combobox_editable.append("Editable Item 1");
    combobox_editable.append("Editable Item 2");
    combobox_editable.append("Editable Item 3");
    combobox_editable.set_value("Custom Text");

    let radiobuttons = RadioButtons::new();
    radiobuttons.append("Radio Button 1");
    radiobuttons.append("Radio Button 2");
    radiobuttons.append("Radio Button 3");

    vbox_lists.append(combobox, LayoutStrategy::Compact);
    vbox_lists.append(combobox_editable, LayoutStrategy::Compact);
    vbox_lists.append(radiobuttons, LayoutStrategy::Compact);

    group_lists.set_child(vbox_lists);
    hbox.append(group_lists.clone(), LayoutStrategy::Stretchy);

    return hbox.into();
}
