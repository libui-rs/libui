//! Text input mechanisms in various forms.
//!
//! All text buffers accept and return `\n` line endings; if on Windows, the appropriate
//! `\r\n` for display are added and removed by the controls.

use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use libui_ffi::{self, uiControl, uiEntry, uiMultilineEntry};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_int;
use std::os::raw::c_void;
use str_tools::{from_toolkit_string, to_toolkit_string};

pub trait TextEntry {
    fn value(&self) -> String;
    fn set_value(&mut self, value: &str);
    fn on_changed<'ctx, F: FnMut(String) + 'static>(&mut self, callback: F);
}

define_control! {
    /// Single-line editable text buffer.
    rust_type: Entry,
    sys_type: uiEntry
}

define_control! {
    /// Single-line editable text buffer.
    rust_type: PasswordEntry,
    sys_type: uiEntry
}

define_control! {
    /// A text buffer suitable for searching.
    rust_type: SearchEntry,
    sys_type: uiEntry
}

define_control! {
    /// Multi-line editable text buffer.
    rust_type: MultilineEntry,
    sys_type: uiMultilineEntry
}

impl Entry {
    pub fn new() -> Entry {
        unsafe { Entry::from_raw(libui_ffi::uiNewEntry()) }
    }
}

impl PasswordEntry {
    pub fn new() -> PasswordEntry {
        unsafe { PasswordEntry::from_raw(libui_ffi::uiNewPasswordEntry()) }
    }
}

impl SearchEntry {
    pub fn new() -> SearchEntry {
        unsafe { SearchEntry::from_raw(libui_ffi::uiNewSearchEntry()) }
    }
}

impl MultilineEntry {
    pub fn new() -> MultilineEntry {
        unsafe { MultilineEntry::from_raw(libui_ffi::uiNewMultilineEntry()) }
    }

    /// Creates an entry that doesn't wrap the text or resize, but has horizontal scrollbars instead.
    pub fn new_nonwrapping() -> MultilineEntry {
        unsafe { MultilineEntry::from_raw(libui_ffi::uiNewNonWrappingMultilineEntry()) }
    }

    pub fn append(&mut self, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { libui_ffi::uiMultilineEntryAppend(self.uiMultilineEntry, cstring.as_ptr()) }
    }

    pub fn readonly(&self) -> bool {
        unsafe { libui_ffi::uiMultilineEntryReadOnly(self.uiMultilineEntry) != 0 }
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        unsafe { libui_ffi::uiMultilineEntrySetReadOnly(self.uiMultilineEntry, readonly as c_int) }
    }
}

impl TextEntry for Entry {
    fn value(&self) -> String {
        unsafe { from_toolkit_string(libui_ffi::uiEntryText(self.uiEntry)) }
    }

    fn set_value(&mut self, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { libui_ffi::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(entry: *mut uiEntry, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let string = unsafe { CStr::from_ptr(libui_ffi::uiEntryText(entry)) }
                .to_string_lossy()
                .into_owned();
            unsafe { from_void_ptr::<G>(data)(string) }
        }

        unsafe {
            libui_ffi::uiEntryOnChanged(self.uiEntry, Some(c_callback::<F>), to_heap_ptr(callback));
        }
    }
}

impl TextEntry for PasswordEntry {
    fn value(&self) -> String {
        unsafe {
            CStr::from_ptr(libui_ffi::uiEntryText(self.uiEntry))
                .to_string_lossy()
                .into_owned()
        }
    }

    fn set_value(&mut self, value: &str) {
        let cstring = CString::new(value.as_bytes().to_vec()).unwrap();
        unsafe { libui_ffi::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F: FnMut(String) + 'static>(&mut self, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(String)>> = Box::new(Box::new(callback));
            libui_ffi::uiEntryOnChanged(
                self.uiEntry,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(String)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiEntry, data: *mut c_void) {
            unsafe {
                let string = from_toolkit_string(libui_ffi::uiEntryText(entry));
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(String)>>(data)(string);
                mem::forget(entry);
            }
        }
    }
}

impl TextEntry for SearchEntry {
    fn value(&self) -> String {
        unsafe { from_toolkit_string(libui_ffi::uiEntryText(self.uiEntry)) }
    }

    fn set_value(&mut self, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { libui_ffi::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    /// Some systems will deliberately delay the callback for a more natural feel.
    fn on_changed<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(entry: *mut uiEntry, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let string = unsafe { CStr::from_ptr(libui_ffi::uiEntryText(entry)) }
                .to_string_lossy()
                .into_owned();
            unsafe { from_void_ptr::<G>(data)(string) }
        }

        unsafe {
            libui_ffi::uiEntryOnChanged(self.uiEntry, Some(c_callback::<F>), to_heap_ptr(callback));
        }
    }
}

impl TextEntry for MultilineEntry {
    fn value(&self) -> String {
        unsafe { from_toolkit_string(libui_ffi::uiMultilineEntryText(self.uiMultilineEntry)) }
    }

    fn set_value(&mut self, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { libui_ffi::uiMultilineEntrySetText(self.uiMultilineEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(entry: *mut uiMultilineEntry, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let string = unsafe { CStr::from_ptr(libui_ffi::uiMultilineEntryText(entry)) }
                .to_string_lossy()
                .into_owned();
            unsafe { from_void_ptr::<G>(data)(string) }
        }

        unsafe {
            libui_ffi::uiMultilineEntryOnChanged(
                self.uiMultilineEntry,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
