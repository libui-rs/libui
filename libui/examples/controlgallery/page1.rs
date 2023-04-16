use libui::controls::*;
use libui::prelude::*;

pub fn make_basic_page(_ui: UI) -> Control {
    let mut vbox = VerticalBox::new();
    vbox.set_padded(true);

    let mut hbox = HorizontalBox::new();
    hbox.set_padded(true);

    let bt = Button::new("Button");
    let cb = Checkbox::new("Checkbox");
    hbox.append(bt, LayoutStrategy::Compact);
    hbox.append(cb, LayoutStrategy::Compact);
    vbox.append(hbox, LayoutStrategy::Compact);

    let lb = Label::new("This is a label.\nLabels can span multiple lines.");
    let hs = HorizontalSeparator::new();
    vbox.append(lb, LayoutStrategy::Compact);
    vbox.append(hs, LayoutStrategy::Compact);

    let mut form = Form::new();
    form.set_padded(true);

    let entry = Entry::new();
    let entry_pwd = PasswordEntry::new();
    let entry_search = SearchEntry::new();
    let entry_multi = MultilineEntry::new();
    let entry_nowrap = MultilineEntry::new_nonwrapping();

    form.append("Entry", entry, LayoutStrategy::Compact);
    form.append("Password Entry", entry_pwd, LayoutStrategy::Compact);
    form.append("Search Entry", entry_search, LayoutStrategy::Compact);
    form.append("Multiline Entry", entry_multi, LayoutStrategy::Stretchy);
    form.append("Non-wrapping Entry", entry_nowrap, LayoutStrategy::Stretchy);

    let mut group = Group::new("Entries");
    group.set_margined(true);
    group.set_child(form);

    vbox.append(group, LayoutStrategy::Stretchy);

    return vbox.into();
}
