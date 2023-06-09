# libui-ffi

Autogenerated bindings to [`libui-ng`](https://github.com/libui-ng/libui-ng) for use in the high-level rust version of [`libui`](https://github.com/libui-rs/libui). It is not meant to be used directly. Compared to other bindings, the goal is to cut down on the buildtool dependencies to ease installation and compilation.

## Prerequisits & Building

Required are compiler, linker, git and the UI platform SDK. No CMake, no Python, no Meson.

__Linux:__
* libgtk-3-dev (debian package) - UI platform headers
* libclang (debian package) - for generating the `libui-ng` bindings.

__Windows:__
* MSVC (via Windows SDK or Visual Studio) - Platform headers, compiler and linker. 
* LLVM - for generating the `libui-ng` bindings.

### Why libclang / LLVM?
Bindgen relies on libclang, but [doesn't ship it](https://github.com/rust-lang/rust-bindgen/issues/918).

### Does MinGW work instead of MSVC?
Not sure. MinGW-64 instead of MSVC does compile and link. libui applications compiled with MinGW-64 versions <= 4.X won't start due to MinGW missing `TaskDialog()`. Later versions should work properly.

## Patches

This crate applies some patches to libui-ng to provide a better experience. Please see the `patches` directory.
* `allow_small_pbar.patch` - The progress bar minimum width is reduced for Windows to better match the behaviour of GTK and Cocoa progress bars.