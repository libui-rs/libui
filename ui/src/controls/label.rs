use super::Control;
use std::ffi::{CStr, CString};
use std::mem;
use ui_sys::{self, uiControl, uiLabel};

define_control! {
    /// A non-interactable piece of text.
    rust_type: Label,
    sys_type: uiLabel
}

impl Label {
    /// Create a new label with the given string as its text.
    /// Note that labels do not auto-wrap their text; they will expand as far as needed
    /// to fit.
    pub fn new(text: &str) -> Label {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Label::from_raw(ui_sys::uiNewLabel(c_string.as_ptr()))
        }
    }

    /// Get a copy of the existing text on the label.
    pub fn text(&self) -> String {
        unsafe {
            CStr::from_ptr(ui_sys::uiLabelText(self.uiLabel))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Get a reference to the existing text on the label.
    pub fn text_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(ui_sys::uiLabelText(self.uiLabel)) }
    }

    /// Set the text on the label.
    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiLabelSetText(self.uiLabel, c_string.as_ptr())
        }
    }
}
