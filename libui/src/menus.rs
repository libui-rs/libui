//! Menus that appear at the top of windows, and the items that go in them.

use callback_helpers::{from_void_ptr, to_heap_ptr};
use controls::Window;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use libui_ffi::{self, uiMenu, uiMenuItem, uiWindow};
use UI;

/// A `MenuItem` represents an item that is shown in a `Menu`. Note that, unlike many controls,
/// the text on `MenuItem`s cannot be changed after creation.
#[derive(Clone)]
pub struct MenuItem {
    ui_menu_item: *mut uiMenuItem,
}

/// A `Menu` represents one of the top-level menus at the top of a window. As that bar is unique
/// per application, creating a new `Menu` shows it on all windows that support displaying menus.
#[derive(Clone)]
pub struct Menu {
    ui_menu: *mut uiMenu,
}

impl MenuItem {
    /// Enables the item, allowing it to be selected. This is the default state of a menu item.
    pub fn enable(&self) {
        unsafe { libui_ffi::uiMenuItemEnable(self.ui_menu_item) }
    }

    /// Disables the item, preventing it from being selected and providing a visual cue to the
    /// user that it cannot be selected.
    pub fn disable(&self) {
        unsafe { libui_ffi::uiMenuItemDisable(self.ui_menu_item) }
    }

    /// Returns `true` if the menu item is checked, and false if it is not checked (or not checkable).
    pub fn checked(&self) -> bool {
        unsafe { libui_ffi::uiMenuItemChecked(self.ui_menu_item) != 0 }
    }

    /// Sets the menu item to either checked or unchecked based on the given value.
    ///
    /// Setting the checked value of a non-checkable menu item has no effect.
    pub fn set_checked(&self, checked: bool) {
        unsafe { libui_ffi::uiMenuItemSetChecked(self.ui_menu_item, checked as c_int) }
    }

    /// Sets the function to be executed when the item is clicked/selected.
    pub fn on_clicked<'ctx, F>(&self, callback: F)
    where
        F: FnMut(&MenuItem, &Window) + 'static,
    {
        extern "C" fn c_callback<G: FnMut(&MenuItem, &Window)>(
            menu_item: *mut uiMenuItem,
            window: *mut uiWindow,
            data: *mut c_void,
        ) {
            let menu_item = unsafe { MenuItem::from_raw(menu_item) };
            let window = unsafe { Window::from_raw(window) };
            unsafe {
                from_void_ptr::<G>(data)(&menu_item, &window);
            }
        }
        unsafe {
            libui_ffi::uiMenuItemOnClicked(
                self.ui_menu_item,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }

    // Creates a `MenuItem` from a raw pointer
    pub unsafe fn from_raw(raw: *mut uiMenuItem) -> Self {
        MenuItem { ui_menu_item: raw }
    }
}

impl Menu {
    /// Creates a new menu with the given name to be displayed in the menubar at the top of the window.
    pub fn new(name: &str) -> Menu {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            Menu {
                ui_menu: libui_ffi::uiNewMenu(c_string.as_ptr()),
            }
        }
    }

    /// Adds a new item with the given name to the menu.
    pub fn append_item(&self, name: &str) -> MenuItem {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            MenuItem {
                ui_menu_item: libui_ffi::uiMenuAppendItem(self.ui_menu, c_string.as_ptr()),
            }
        }
    }

    /// Adds a new togglable (checkbox) item with the given name to the menu.
    pub fn append_check_item(&self, name: &str) -> MenuItem {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            MenuItem {
                ui_menu_item: libui_ffi::uiMenuAppendCheckItem(self.ui_menu, c_string.as_ptr()),
            }
        }
    }

    /// Adds a seperator to the menu.
    pub fn append_separator(&self) {
        unsafe { libui_ffi::uiMenuAppendSeparator(self.ui_menu) }
    }
}
