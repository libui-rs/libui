use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use libui_ffi::{self, uiControl, uiDateTimePicker};
use std::mem;
use std::os::raw::c_void;
use ui::UI;

define_control! {
    /// Allows to enter a date and/or time.
    rust_type: DateTimePicker,
    sys_type: uiDateTimePicker
}

/// The possible selections the `DateTimePicker` shall offer.
pub enum DateTimePickerKind {
    DateTime,
    Date,
    Time,
}

impl DateTimePicker {
    /// Create a new date and/or time picker.
    pub fn new(mode: DateTimePickerKind) -> DateTimePicker {
        unsafe {
            DateTimePicker::from_raw(match mode {
                DateTimePickerKind::DateTime => libui_ffi::uiNewDateTimePicker(),
                DateTimePickerKind::Date => libui_ffi::uiNewDatePicker(),
                DateTimePickerKind::Time => libui_ffi::uiNewTimePicker(),
            })
        }
    }

    /// Returns the date and/or time stored in the DateTimePicker.
    ///
    /// Depending on the `DateTimePickerKind` you created, the date or time fields
    /// will not be set and instead contain their unix epoch default.
    ///
    /// Warning: The `struct tm` member `tm_isdst` is unused on Windows and will be `-1`.
    pub fn datetime(&self) -> libc::tm {
        unsafe {
            // `struct tm` varies depending on platform (tm_gmtoff, tm_zone). We thus can't
            // init the fields ourselves but have to memset init.
            let mut datetime = std::mem::MaybeUninit::<libc::tm>::zeroed().assume_init();

            let ptr = &mut datetime as *mut libc::tm;
            libui_ffi::uiDateTimePickerTime(self.uiDateTimePicker, ptr as *mut libui_ffi::tm);
            datetime
        }
    }

    /// Sets date and time of the DateTimePicker.
    ///
    /// Warning: The `struct tm` member `tm_isdst` is ignored on Windows and should be set to `-1`
    pub fn set_datetime(&self, datetime: libc::tm) {
        unsafe {
            let ptr = &datetime as *const libc::tm;
            libui_ffi::uiDateTimePickerSetTime(self.uiDateTimePicker, ptr as *const libui_ffi::tm);
        }
    }

    /// Registers a callback for when the date time picker value is changed by the user.
    ///
    /// The callback is not triggered when calling `set_datetime()`.
    /// Only one callback can be registered at a time.
    pub fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(&mut DateTimePicker) + 'static,
    {
        extern "C" fn c_callback<G>(picker: *mut uiDateTimePicker, data: *mut c_void)
        where
            G: FnMut(&mut DateTimePicker),
        {
            let mut picker = DateTimePicker {
                uiDateTimePicker: picker,
            };
            unsafe {
                from_void_ptr::<G>(data)(&mut picker);
            }
        }
        unsafe {
            libui_ffi::uiDateTimePickerOnChanged(
                self.uiDateTimePicker,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
