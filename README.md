# libui [![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT) [![License: Apache](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


## The native cross-platform UI toolkit for Rust

`libui` is a **simple**, **small** and **easy to distribute** GUI library. It provides a native UI for your platform by utilising your systems API instead of implementing yet another mismatched looking renderer. Thus all `libui` apps are lightweight, have a native look and feel and start from a highly performant base which is well integrated with the ecosystem on each platform. Because it implements only the common subset of these platform APIs, your apps will work on all platforms and won't have significant behavioral inconsistencies, with no additional effort on your part.

Technically, `libui` is a "rustification" wrapper over the C library [`libui-ng`](https://github.com/libui-ng/libui-ng), which actually abstracts the native GUI framework. That is the Win32 API on Windows, Cocoa on Mac OS X, and GTK3 for Linux and others. 

## Using

Add `libui` to your dependency list in `cargo.toml` with:

```toml
libui = { git = "https://github.com/libui-rs/libui" }
```

Next we suggest to have a look at the [example applications](https://github.com/libui-rs/libui/ui/examples) or start with the minimal example printed here:

```rust
#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

extern crate libui;
use libui::controls::*;
use libui::prelude::*;

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

We also have documentation on [docs.rs](https://docs.rs/libui) for all released versions.

## Screenshots

On the left see a `libui` application running on a GNOME desktop with GTK 3. On the right under Windows 10 with its native user interface.

![Example application running under Linux and Windows](images/libui_gtk_win.png)

For more screenshots, see [`here`](https://github.com/nptr/libui-rs/tree/development/images).

## Prerequisits & Building

To build the underlying `libui-ng` your system must have certain tools installed in addition to your rust toolchain. I am trying to cut down on the inconveniences as far as possible.

__Linux:__
* libgtk-3-dev (debian package) - for compiling `libui-ng`
* libclang (debian package) - for generating the `libui-ng` bindings

__Windows:__
* MSVC (via Windows SDK or Visual Studio)
* LLVM - for generating the `libui-ng` bindings

Note: MinGW-64 does compile and link, but the application won't start due to MinGW missing `TaskDialog()`. Reportedly, with versions >= 5.X, the function is available.

## Acknowledgments

* Initial work from [@pcwalton](https://github.com/pcwalton/), who largely shaped this library.
* Further development was done at [rust-native-ui/libui-rs](https://github.com/rust-native-ui/libui-rs) by [@NoraCodes](https://github.com/NoraCodes/) and others.
* This library is the continuation of the above, but using `libui-ng` with its new features instead the original, now abandoned, same-named C library `libui`.