//! `libui` is a **simple**, **small** and **easy to distribute** GUI library. It provides a native UI for your platform by utilising your systems API instead of implementing yet another mismatched looking renderer.
//! Technically, `libui` is a "rustification" wrapper over the C library [`libui-ng`](https://github.com/libui-ng/libui-ng), which actually abstracts the native GUI framework. That is the Win32 API on Windows, Cocoa on Mac OS X, and GTK3 for Linux and others. 
//! 
//! Add `libui` to your dependency list with:
//!
//! ```toml
//! libui = { git = "https://github.com/libui-rs/libui" }
//! ```
//! 
//! Next we suggest to have a look at the [example applications](https://github.com/libui-rs/libui/tree/development/libui/examples) or start with the minimal example printed here:
//! 
//! ```no_run
//! #![cfg_attr(not(test), windows_subsystem = "windows")]
//! #![cfg_attr(test, windows_subsystem = "console")]
//! 
//! extern crate libui;
//! use libui::controls::*;
//! use libui::prelude::*;
//! 
//! fn main() {
//!     let ui = UI::init()
//!         .expect("Couldn't initialize UI library");
//!   
//!     let mut win = Window::new(&ui, "Example", 300, 200, 
//!         WindowType::NoMenubar);
//!     let layout = VerticalBox::new();
//! 
//!     // add controls to your layout here
//! 
//!     win.set_child(layout);
//!     win.show();
//!     ui.main();
//! }
//! ```
//!

#[macro_use]
extern crate bitflags;

extern crate libc;
extern crate libui_ffi;

mod callback_helpers;
mod compile_tests;
pub mod controls;
pub mod draw;
mod error;
mod ffi_tools;
pub mod menus;
pub mod str_tools;
mod ui;

pub use error::UIError;
pub use ui::{EventLoop, UI};

/// Common imports are packaged into this module. It's meant to be glob-imported: `use libui::prelude::*`.
pub mod prelude {
    pub use controls::LayoutStrategy;
    pub use controls::{NumericEntry, TextEntry};
    pub use controls::{Window, WindowType};
    pub use ui::UI;
}
