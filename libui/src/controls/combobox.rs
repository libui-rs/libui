use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::ffi::CStr;
use std::i32;
use std::mem;
use std::os::raw::c_void;
use str_tools::to_toolkit_string;
use ui::UI;
use libui_ffi::{self, uiCombobox, uiControl, uiEditableCombobox};

define_control! {
    /// Allows the user to select any one of its options, from a list shown only when selected.
    rust_type: Combobox,
    sys_type: uiCombobox
}

impl Combobox {
    /// Create a new Combobox
    pub fn new() -> Self {
        unsafe { Combobox::from_raw(libui_ffi::uiNewCombobox()) }
    }

    /// Adds a new option to the combo box.
    pub fn append(&self, name: &str) {
        unsafe {
            let c_string = to_toolkit_string(name);
            libui_ffi::uiComboboxAppend(self.uiCombobox, c_string.as_ptr())
        }
    }

    /// Inserts an item at index to the combo box.
    pub fn insert_at(&self, index: i32, name: &str) {
        unsafe {
            let c_string = to_toolkit_string(name);
            libui_ffi::uiComboboxInsertAt(self.uiCombobox, index, c_string.as_ptr())
        }
    }

    /// Deletes the item at index from the combo box.
    ///
    /// Deleting the index of the item currently selected will move the
    /// selection to the next item in the combo box or `-1` if no such item exists.
    pub fn delete(&self, index: i32) {
        unsafe { libui_ffi::uiComboboxDelete(self.uiCombobox, index) }
    }

    /// Deletes all items from the combo box.
    pub fn clear(&self) {
        unsafe { libui_ffi::uiComboboxClear(self.uiCombobox) }
    }

    /// Returns the number of items contained within the combo box.
    pub fn count(&self) -> i32 {
        unsafe { libui_ffi::uiComboboxNumItems(self.uiCombobox) }
    }

    /// Returns the index of the currently selected option.
    pub fn selected(&self) -> i32 {
        unsafe { libui_ffi::uiComboboxSelected(self.uiCombobox) }
    }

    /// Selects an item by its index
    pub fn set_selected(&mut self, value: i32) {
        unsafe { libui_ffi::uiComboboxSetSelected(self.uiCombobox, value) }
    }

    pub fn on_selected<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        extern "C" fn c_callback<G>(combobox: *mut uiCombobox, data: *mut c_void)
        where
            G: FnMut(i32),
        {
            let val = unsafe { libui_ffi::uiComboboxSelected(combobox) };
            unsafe { from_void_ptr::<G>(data)(val) }
        }

        unsafe {
            libui_ffi::uiComboboxOnSelected(
                self.uiCombobox,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}

define_control! {
    /// A control to select one item from a predefined list of items or enter ones own.
    rust_type: EditableCombobox,
    sys_type: uiEditableCombobox
}

impl EditableCombobox {
    /// Creates a new editable combobox.
    pub fn new() -> EditableCombobox {
        unsafe { EditableCombobox::from_raw(libui_ffi::uiNewEditableCombobox()) }
    }

    /// Adds a new option to the combobox.
    pub fn append(&self, name: &str) {
        unsafe {
            let c_string = to_toolkit_string(name);
            libui_ffi::uiEditableComboboxAppend(self.uiEditableCombobox, c_string.as_ptr())
        }
    }

    // Returns the text of the editable combobox.
    //
    // This text is either the text of one of the predefined
    // list items or the text manually entered by the user.
    pub fn value(&self) -> String {
        let ptr = unsafe { libui_ffi::uiEditableComboboxText(self.uiEditableCombobox) };
        let text: String = unsafe { CStr::from_ptr(ptr).to_string_lossy().into() };
        unsafe {
            libui_ffi::uiFreeText(ptr);
        }
        text
    }

    // Sets the editable combobox text.
    pub fn set_value(&mut self, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { libui_ffi::uiEditableComboboxSetText(self.uiEditableCombobox, cstring.as_ptr()) }
    }

    /// Registers a callback for when an editable combobox item is selected or user text changed.
    ///
    /// Note: The callback is not triggered when calling set_value().
    /// Note: Only one callback can be registered at a time.
    pub fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(combobox: *mut uiEditableCombobox, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let ptr = unsafe { libui_ffi::uiEditableComboboxText(combobox) };
            let text: String = unsafe { CStr::from_ptr(ptr).to_string_lossy().into() };
            unsafe {
                libui_ffi::uiFreeText(ptr);
            }
            unsafe { from_void_ptr::<G>(data)(text) }
        }

        unsafe {
            libui_ffi::uiEditableComboboxOnChanged(
                self.uiEditableCombobox,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
