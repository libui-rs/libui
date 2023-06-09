use super::Control;
use std::ffi::CString;
use std::i32;
use std::mem;
use std::os::raw::c_void;
use ui::UI;
use libui_ffi::{self, uiControl, uiRadioButtons};

define_control! {
    /// A set of toggles; only one can be selected at a time.
    rust_type: RadioButtons,
    sys_type: uiRadioButtons
}

impl RadioButtons {
    pub fn new() -> Self {
        unsafe { RadioButtons::from_raw(libui_ffi::uiNewRadioButtons()) }
    }

    pub fn append(&self, name: &str) {
        let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
        unsafe {
            libui_ffi::uiRadioButtonsAppend(self.uiRadioButtons, c_string.as_ptr());
        }
    }

    pub fn selected(&self) -> i32 {
        unsafe { libui_ffi::uiRadioButtonsSelected(self.uiRadioButtons) }
    }

    pub fn set_selected(&mut self, idx: i32) {
        unsafe {
            libui_ffi::uiRadioButtonsSetSelected(self.uiRadioButtons, idx);
        }
    }

    pub fn on_selected<'ctx, F: FnMut(i32) + 'static>(&self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
            libui_ffi::uiRadioButtonsOnSelected(
                self.uiRadioButtons,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(i32)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(radio_buttons: *mut uiRadioButtons, data: *mut c_void) {
            unsafe {
                let val = libui_ffi::uiRadioButtonsSelected(radio_buttons);
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(i32)>>(data)(val);
            }
        }
    }
}
