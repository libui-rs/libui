mod page1;
mod page2;
mod page3;
mod page4;

extern crate ui;
use ui::controls::*;
use ui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    let mut layout = HorizontalBox::new();
    let window = Rc::new(RefCell::new(Window::new(
        &ui,
        "Test",
        640,
        480,
        WindowType::NoMenubar,
    )));

    let page1 = page1::make_basic_page(ui.clone());
    let page2 = page2::make_numbers_page(ui.clone());
    let page3 = page3::make_data_page(ui.clone(), window.clone());
    let page4 = page4::make_table_page(ui.clone());

    let mut tabs = TabGroup::new();
    tabs.append("Basic Controls", page1);
    tabs.append("Numbers and Lists", page2);
    tabs.append("Data Choosers", page3);
    tabs.append("Table", page4);

    layout.append(tabs, LayoutStrategy::Stretchy);

    window.borrow_mut().set_child(layout);
    window.borrow_mut().show();

    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(move || {});
    event_loop.run();
}
