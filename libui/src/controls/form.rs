use super::{Control, LayoutStrategy};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_int;
use libui_ffi::{self, uiControl, uiForm};

define_control! {
    /// A container that labels its childen.
    ///
    /// Labels and controls are organized into two panes, making both labels
    /// and controls align with each other. Perfect to create aesthetically
    /// pleasing input forms.
    rust_type: Form,
    sys_type: uiForm
}

impl Form {
    /// Create a new Form
    pub fn new() -> Form {
        unsafe { Form::from_raw(libui_ffi::uiNewForm()) }
    }

    /// Appends a control with a label to the form.
    pub fn append<T: Into<Control>>(&mut self, label: &str, child: T, strategy: LayoutStrategy) {
        let stretchy = match strategy {
            LayoutStrategy::Compact => false,
            LayoutStrategy::Stretchy => true,
        };
        let control = child.into();
        unsafe {
            let c_string = CString::new(label.as_bytes().to_vec()).unwrap();
            // TODO: have ctx as member?
            //assert!(ctx.parent_of(control.clone()).is_none());
            libui_ffi::uiFormAppend(
                self.uiForm,
                c_string.as_ptr(),
                control.ui_control,
                stretchy as c_int,
            )
        }
    }

    /// Returns the number of controls contained within the form.
    pub fn count(&self) -> i32 {
        unsafe { libui_ffi::uiFormNumChildren(self.uiForm) }
    }

    /// Removes the control at `index` from the form.
    pub fn delete(&mut self, index: i32) {
        unsafe { libui_ffi::uiFormDelete(self.uiForm, index) }
    }

    /// Returns whether or not controls within the form are padded.
    pub fn padded(&self) -> bool {
        unsafe { libui_ffi::uiFormPadded(self.uiForm) != 0 }
    }

    /// Sets whether or not controls within the form are padded.
    ///
    /// Padding is defined as space between individual controls.
    /// The padding size is determined by the OS defaults.
    pub fn set_padded(&mut self, padded: bool) {
        unsafe {
            libui_ffi::uiFormSetPadded(self.uiForm, padded as c_int);
        }
    }
}
