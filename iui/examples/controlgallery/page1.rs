
extern crate iui;
use iui::controls::*;
use iui::prelude::*;

pub fn make_basic_page(ui: UI) -> Control {
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);

    let bt = Button::new(&ui, "Button");
    let cb = Checkbox::new(&ui, "Checkbox");
    hbox.append(&ui, bt, LayoutStrategy::Compact);
    hbox.append(&ui, cb, LayoutStrategy::Compact);
    vbox.append(&ui, hbox, LayoutStrategy::Compact);

    let lb = Label::new(&ui, "This is a label.\nLabels can span multiple lines.");
    let hs = HorizontalSeparator::new(&ui);
    vbox.append(&ui, lb, LayoutStrategy::Compact);
    vbox.append(&ui, hs, LayoutStrategy::Compact);

    let mut form = Form::new(&ui);
    form.set_padded(&ui, true);

    let entry = Entry::new(&ui);
    let entry_pwd = PasswordEntry::new(&ui);
    let entry_search = SearchEntry::new(&ui);
    let entry_multi = MultilineEntry::new(&ui);
    let entry_nowrap = MultilineEntry::new(&ui);

    form.append(&ui, "Entry", entry, LayoutStrategy::Compact);
    form.append(&ui, "Password Entry", entry_pwd, LayoutStrategy::Compact);
    form.append(&ui, "Search Entry", entry_search, LayoutStrategy::Compact);
    form.append(&ui, "Multiline Entry", entry_multi, LayoutStrategy::Compact);
    form.append(&ui, "Non-wrapping Entry", entry_nowrap, LayoutStrategy::Compact);
        
    let mut group = Group::new(&ui, "Entries");
    group.set_margined(&ui, true);
    group.set_child(&ui, form);

    vbox.append(&ui, group, LayoutStrategy::Stretchy);

    return vbox.into();
}