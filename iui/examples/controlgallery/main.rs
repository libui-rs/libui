mod page1;
mod page2;
mod page3;
mod page4;

extern crate iui;
use std::cell::RefCell;
use iui::controls::*;
use iui::prelude::*;
use std::rc::Rc;

fn main() {
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    let mut layout = HorizontalBox::new(&ui);
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

    let mut tabs = TabGroup::new(&ui);
    tabs.append(&ui, "Basic Controls", page1);
    tabs.append(&ui, "Numbers and Lists", page2);
    tabs.append(&ui, "Data Choosers", page3);
    tabs.append(&ui, "Table", page4);

    layout.append(&ui, tabs, LayoutStrategy::Stretchy);

    window.borrow_mut().set_child(&ui, layout);
    window.borrow_mut().show(&ui);

    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        move || {}
    });
    event_loop.run(&ui);
}
