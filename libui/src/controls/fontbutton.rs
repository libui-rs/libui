use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::{convert::TryFrom, mem};
use ui::UI;
use libui_ffi::{self, uiControl, uiFontButton, uiFontDescriptor};

/// An enum listing font styles.
#[derive(Copy, Clone, Debug)]
pub enum SlantStyle {
    Normal,
    Oblique,
    Italic,
}

impl TryFrom<u32> for SlantStyle {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SlantStyle::Normal),
            1 => Ok(SlantStyle::Oblique),
            2 => Ok(SlantStyle::Italic),
            _ => Err(()),
        }
    }
}

/// An enum listing font stretch levels.
#[derive(Copy, Clone, Debug)]
pub enum StretchStyle {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl TryFrom<u32> for StretchStyle {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StretchStyle::UltraCondensed),
            1 => Ok(StretchStyle::ExtraCondensed),
            2 => Ok(StretchStyle::Condensed),
            3 => Ok(StretchStyle::SemiCondensed),
            4 => Ok(StretchStyle::Normal),
            5 => Ok(StretchStyle::SemiExpanded),
            6 => Ok(StretchStyle::Expanded),
            7 => Ok(StretchStyle::ExtraExpanded),
            8 => Ok(StretchStyle::UltraExpanded),
            _ => Err(()),
        }
    }
}

/// A structure describing a font by it properties.
#[derive(Debug, Clone)]
pub struct FontDescription {
    pub family: String,
    pub size: f64,
    pub weight: u32,
    pub slant: SlantStyle,
    pub stretch: StretchStyle,
}

impl Default for FontDescription {
    fn default() -> Self {
        FontDescription {
            family: String::new(),
            size: 0.0,
            weight: 0,
            slant: SlantStyle::Normal,
            stretch: StretchStyle::Normal,
        }
    }
}

define_control! {
    /// A button which allows the user to pick a font.
    rust_type: FontButton,
    sys_type: uiFontButton
}

impl FontButton {
    /// Creates a new font button.
    pub fn new() -> FontButton {
        unsafe { FontButton::from_raw(libui_ffi::uiNewFontButton()) }
    }

    /// Returns the selected font.
    pub fn font(&self) -> FontDescription {
        unsafe {
            let mut cfd: uiFontDescriptor = { MaybeUninit::zeroed().assume_init() };

            libui_ffi::uiFontButtonFont(self.uiFontButton, &mut cfd as *mut uiFontDescriptor);

            let fd = FontDescription {
                family: CStr::from_ptr(cfd.Family).to_string_lossy().into(),
                size: cfd.Size,
                weight: cfd.Weight,
                slant: SlantStyle::try_from(cfd.Italic).unwrap(),
                stretch: StretchStyle::try_from(cfd.Stretch).unwrap(),
            };

            libui_ffi::uiFreeFontButtonFont(&mut cfd as *mut uiFontDescriptor);
            return fd;
        }
    }

    /// Registers a callback for when the font is changed.
    ///
    /// Note: Only one callback can be registered at a time.
    pub fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(&mut FontButton) + 'static,
    {
        extern "C" fn c_callback<G>(button: *mut uiFontButton, data: *mut c_void)
        where
            G: FnMut(&mut FontButton),
        {
            let mut button = FontButton {
                uiFontButton: button,
            };
            unsafe {
                from_void_ptr::<G>(data)(&mut button);
            }
        }
        unsafe {
            libui_ffi::uiFontButtonOnChanged(
                self.uiFontButton,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
