
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

    let mut grid = LayoutGrid::new(&ui);
    grid.set_padded(&ui, true);

    // Column 0 - Label
    {
        let lb_entry = Label::new(&ui, "Entry");
        let lb_entry_pwd = Label::new(&ui, "Password Entry");
        let lb_entry_search = Label::new(&ui, "Search Entry");
        let lb_entry_multi = Label::new(&ui, "Multiline Entry");
        let lb_entry_nowrap = Label::new(&ui, "Non-wrapping Entry");

        grid.append(
            &ui,
            lb_entry.clone(),
            0,
            0,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            lb_entry_pwd.clone(),
            0,
            1,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            lb_entry_search.clone(),
            0,
            2,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            lb_entry_multi.clone(),
            0,
            3,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            lb_entry_nowrap.clone(),
            0,
            4,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
    };

    // Column 1 - Inputs
    let (entry, entry_pwd, entry_search, entry_multi, entry_nowrap) = {
        let entry = Entry::new(&ui);
        let entry_pwd = PasswordEntry::new(&ui);
        let entry_search = SearchEntry::new(&ui);
        let entry_multi = MultilineEntry::new(&ui);
        let entry_nowrap = MultilineEntry::new(&ui);

        grid.append(
            &ui,
            entry.clone(),
            1,
            0,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            entry_pwd.clone(),
            1,
            1,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            entry_search.clone(),
            1,
            2,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            entry_multi.clone(),
            1,
            3,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );
        grid.append(
            &ui,
            entry_nowrap.clone(),
            1,
            4,
            1,
            1,
            GridExpand::Neither,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        (entry, entry_pwd, entry_search, entry_multi, entry_nowrap)
    };

    let mut group = Group::new(&ui, "Entries");
    group.set_margined(&ui, true);
    group.set_child(&ui, grid);

    vbox.append(&ui, group, LayoutStrategy::Stretchy);

    return vbox.into();
}