use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::mem;
use std::os::raw::c_void;
use ui::UI;
use libui_ffi::{self, uiColorButton, uiControl};

define_control! {
    /// A button-like control which allows the user to pick a color.
    rust_type: ColorButton,
    sys_type: uiColorButton
}

impl ColorButton {
    /// Create a new color button.
    pub fn new() -> ColorButton {
        unsafe { ColorButton::from_raw(libui_ffi::uiNewColorButton()) }
    }

    /// Get the selected color as RGBA with values in range of [0, 1.0] per component.
    pub fn color(&self) -> (f64, f64, f64, f64) {
        unsafe {
            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
            libui_ffi::uiColorButtonColor(self.uiColorButton, &mut r, &mut g, &mut b, &mut a);
            (r, g, b, a)
        }
    }

    /// Set the buttons selected color. Component are in the range of [0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        unsafe {
            libui_ffi::uiColorButtonSetColor(self.uiColorButton, r, g, b, a);
        }
    }

    /// Run the given callback when the selected color changed.
    ///
    /// The callback is not triggered when calling `set_color()`.
    /// Only one callback can be registered at a time.
    pub fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(&mut ColorButton) + 'static,
    {
        extern "C" fn c_callback<G>(button: *mut uiColorButton, data: *mut c_void)
        where
            G: FnMut(&mut ColorButton),
        {
            let mut button = ColorButton {
                uiColorButton: button,
            };
            unsafe {
                from_void_ptr::<G>(data)(&mut button);
            }
        }
        unsafe {
            libui_ffi::uiColorButtonOnChanged(
                self.uiColorButton,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
