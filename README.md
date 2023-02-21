# Improved User Interface [![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT) [![License: Apache](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


## A native cross-platform UI toolkit for Rust

`iui` is a **simple**, **small** and **easy to distribute** GUI library. It provides a native UI for your platform by utilising your systems API instead of implementing yet another mismatched looking renderer. Thus all `iui` apps are lightweight, have a native look and feel and start from a highly performant base which is well integrated with the ecosystem on each platform. Because it implements only the least common subset of these platform APIs, your apps will work on all platforms and won't have significant behavioral inconsistencies, with no additional effort on your part.

Technically, `iui` is a "rustification" wrapper over [`libui-ng`](https://github.com/libui-ng/libui-ng) which actually abstracts the native GUI framework. That is the Win32 API on Windows, Cocoa on Mac OS X, and GTK3 for Linux and others. 

## Using

Add `iui` to your dependency list in `cargo.toml` with:

```toml
iui = { git = "https://github.com/nptr/libui-rs", branch = "trunk" }
```

Next we suggest to have a look at the [example applications](https://github.com/nptr/libui-rs/tree/trunk/iui/examples) or start with the minimal example printed here:


```rust
#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

extern crate iui;
use iui::controls::*;
use iui::prelude::*;

fn main() {
    let ui = UI::init()
        .expect("Couldn't initialize UI library");
    
    let mut win = Window::new(&ui, "Example", 300, 200, 
        WindowType::NoMenubar);
    let layout = VerticalBox::new();

    // add controls to your layout here

    win.set_child(layout);
    win.show();
    ui.main();
}
```

We have documentation on [docs.rs](https://docs.rs/iui) for released versions and on [github](https://rust-native-ui.github.io/libui-rs/iui/index.html) for `trunk`.

## Screenshots

On the left see a `libui` application running on a GNOME desktop with GTK 3. On the right under Windows 10 with its native user interface.

![Example application running under Linux and Windows](images/libui_gtk_win.png)

For more screenshots, look at the examples of [`libui-ng`](https://github.com/libui-ng/libui-ng).

## Prerequisits & Building

To build the underlying `libui-ng` your system must have certain tools installed in addition to your rust toolchain. 
The build of `libui-ng` happens automatically, but the tools must be there. I am trying to cut down on this inconvenience.

__Linux:__
* libgtk-3-dev (debian package) - for compiling `libui-ng`
* libclang (debian package) - for generating the `libui-ng` bindings

__Windows:__
* MSVC (via Windows SDK or Visual Studio)
* LLVM - for generating the `libui-ng` bindings

Note: MinGW-64 does compile and link, but the application won't start due to MinGW missing `TaskDialog()`.

## Acknowledgments

* Based on work by [@pcwalton](https://github.com/pcwalton/).
* Later forked from [https://github.com/rust-native-ui/libui-rs](rust-native-ui/libui-rs).