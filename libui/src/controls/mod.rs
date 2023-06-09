//! Available user interface controls and related functionality.
//!
//! Note that `Control` and all specific control types are references to memory which is owned by the UI library.

use ui::UI;
use libui_ffi::{self, uiControl};

use std::ptr;

#[macro_use]
mod create_macro;
mod area;
pub use self::area::*;
mod button;
pub use self::button::*;
mod checkbox;
pub use self::checkbox::*;
mod colorbutton;
pub use self::colorbutton::*;
mod combobox;
pub use self::combobox::*;
mod datetimepicker;
pub use self::datetimepicker::*;
mod fontbutton;
pub use self::fontbutton::*;
mod form;
pub use self::form::*;
mod label;
pub use self::label::*;
mod layout;
pub use self::layout::*;
mod numericentry;
pub use self::numericentry::*;
mod progressbar;
pub use self::progressbar::*;
mod radiobuttons;
pub use self::radiobuttons::*;
mod table;
pub use self::table::*;
mod textentry;
pub use self::textentry::*;
mod window;
pub use self::window::*;

/// A generic UI control. Any UI control can be turned into this type.
///
/// Note that `Control` and all specific control types are references
/// whose memory is owned by the UI library.
pub struct Control {
    ui_control: *mut uiControl,
}

impl Drop for Control {
    fn drop(&mut self) {
        // For now this does nothing, but in the future, when `libui` supports proper memory
        // management, this will likely need to twiddle reference counts.
    }
}

impl Clone for Control {
    fn clone(&self) -> Control {
        Control {
            ui_control: self.ui_control,
        }
    }
}

impl Control {
    /// Creates a new `Control` object from an existing `*mut uiControl`.
    pub unsafe fn from_ui_control(ui_control: *mut uiControl) -> Control {
        Control { ui_control }
    }

    /// Returns the underlying `*mut uiControl`.
    pub fn as_ui_control(&self) -> *mut uiControl {
        self.ui_control
    }

    /// Destroys a control. Any use of the control after this is use-after-free; therefore, this
    /// is marked unsafe.
    pub unsafe fn destroy(&self) {
        // Don't check for initialization here since this can be run during deinitialization.
        libui_ffi::uiControlDestroy(self.ui_control)
    }
}

impl UI {
    // Return the parent control of the given control, or None if the control is orphaned.
    pub fn parent_of<T: Into<Control>>(&self, control: T) -> Option<Control> {
        unsafe {
            let ptr = libui_ffi::uiControlParent(control.into().ui_control);
            if ptr.is_null() {
                None
            } else {
                Some(Control::from_ui_control(ptr))
            }
        }
    }

    /// Set the parent control of this control, "moving" it to a new place in
    /// the UI tree or, if passed `None`, removing it from the tree.
    // TODO: Does this actually need to be unsafe? I don't really see why it is.
    pub unsafe fn set_parent_of<T: Into<Control>>(&mut self, control: T, parent: Option<T>) {
        libui_ffi::uiControlSetParent(
            control.into().ui_control,
            match parent {
                None => ptr::null_mut(),
                Some(parent) => parent.into().ui_control,
            },
        )
    }

    /// Returns true if this control is a top-level control; the root of
    /// the UI tree.
    pub fn is_toplevel<T: Into<Control>>(&self, control: T) -> bool {
        unsafe { libui_ffi::uiControlToplevel(control.into().ui_control) != 0 }
    }

    /// Returns true if this control is currently set to be displayed.
    pub fn is_shown<T: Into<Control>>(&self, control: T) -> bool {
        unsafe { libui_ffi::uiControlVisible(control.into().ui_control) != 0 }
    }

    /// Sets whether or not the control should be displayed.
    pub fn set_shown<T: Into<Control>>(&mut self, control: T, show: bool) {
        if show {
            unsafe { libui_ffi::uiControlShow(control.into().ui_control) }
        } else {
            unsafe { libui_ffi::uiControlHide(control.into().ui_control) }
        }
    }

    /// Returns true if the control is enabled (can be interacted with).
    pub fn is_enabled<T: Into<Control>>(&self, control: T) -> bool {
        unsafe { libui_ffi::uiControlEnabled(control.into().ui_control) != 0 }
    }

    /// Sets the enable/disable state of the control. If disabled, a control
    /// cannot be interacted with, and visual cues to that effect are presented
    /// to the user.
    pub fn set_enabled<T: Into<Control>>(&mut self, control: T, enabled: bool) {
        if enabled {
            unsafe { libui_ffi::uiControlEnable(control.into().ui_control) }
        } else {
            unsafe { libui_ffi::uiControlDisable(control.into().ui_control) }
        }
    }
}
