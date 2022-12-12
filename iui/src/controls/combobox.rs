use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::i32;
use std::mem;
use std::os::raw::c_void;
use str_tools::to_toolkit_string;
use ui::UI;
use ui_sys::{self, uiCombobox, uiControl};

define_control! {
    /// Allows the user to select any one of its options, from a list shown only when selected.
    rust_type: Combobox,
    sys_type: uiCombobox
}

impl Combobox {
    /// Create a new Combobox
    pub fn new(_ctx: &UI) -> Self {
        unsafe { Combobox::from_raw(ui_sys::uiNewCombobox()) }
    }

    /// Adds a new option to the combo box.
    pub fn append(&self, _ctx: &UI, name: &str) {
        unsafe {
            let c_string = to_toolkit_string(name);
            ui_sys::uiComboboxAppend(self.uiCombobox, c_string.as_ptr())
        }
    }

    /// Inserts an item at index to the combo box.
    pub fn insert_at(&self, _ctx: &UI, index: i32, name: &str) {
        unsafe {
            let c_string = to_toolkit_string(name);
            ui_sys::uiComboboxInsertAt(self.uiCombobox, index, c_string.as_ptr())
        }
    }

    /// Deletes the item at index from the combo box.
    ///
    /// Deleting the index of the item currently selected will move the
    /// selection to the next item in the combo box or `-1` if no such item exists.
    pub fn delete(&self, _ctx: &UI, index: i32) {
        unsafe { ui_sys::uiComboboxDelete(self.uiCombobox, index) }
    }

    /// Deletes all items from the combo box.
    pub fn clear(&self, _ctx: &UI) {
        unsafe { ui_sys::uiComboboxClear(self.uiCombobox) }
    }

    /// Returns the number of items contained within the combo box.
    pub fn count(&self, _ctx: &UI) -> i32 {
        unsafe { ui_sys::uiComboboxNumItems(self.uiCombobox) }
    }

    /// Returns the index of the currently selected option.
    pub fn selected(&self, _ctx: &UI) -> i32 {
        unsafe { ui_sys::uiComboboxSelected(self.uiCombobox) }
    }

    /// Selects an item by its index
    pub fn set_selected(&mut self, _ctx: &UI, value: i32) {
        unsafe { ui_sys::uiComboboxSetSelected(self.uiCombobox, value) }
    }

    pub fn on_selected<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        extern "C" fn c_callback<G>(combobox: *mut uiCombobox, data: *mut c_void)
        where
            G: FnMut(i32),
        {
            let val = unsafe { ui_sys::uiComboboxSelected(combobox) };
            unsafe { from_void_ptr::<G>(data)(val) }
        }

        unsafe {
            ui_sys::uiComboboxOnSelected(
                self.uiCombobox,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
