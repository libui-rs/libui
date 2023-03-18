//! Functions and types related to 2D vector graphics.

mod brush;
mod context;
mod path;
mod strokeparams;
mod transform;

pub use self::brush::*;
pub use self::context::*;
pub use self::path::*;
pub use self::strokeparams::*;
pub use self::transform::*;

pub use libui_ffi::uiDrawDefaultMiterLimit as DEFAULT_MITER_LIMIT;

// pub struct FontFamilies {
//     ui_draw_font_families: *mut uiDrawFontFamilies,
// }

// impl Drop for FontFamilies {
//     fn drop(&mut self) {
//         unsafe { libui_ffi::uiDrawFreeFontFamilies(self.ui_draw_font_families) }
//     }
// }

// impl FontFamilies {
//     pub fn list() ->  FontFamilies {
//         unsafe {
//             FontFamilies {
//                 ui_draw_font_families: libui_ffi::uiDrawListFontFamilies(),
//             }
//         }
//     }

//     pub fn len(&self) -> u64 {
//         unsafe { libui_ffi::uiDrawFontFamiliesNumFamilies(self.ui_draw_font_families) }
//     }

//     pub fn family(&self, ctx: &UI, index: u64) -> Text {
//         assert!(index < self.len(ctx));
//         unsafe {
//             Text::new(libui_ffi::uiDrawFontFamiliesFamily(
//                 self.ui_draw_font_families,
//                 index,
//             ))
//         }
//     }
// }

// pub mod text {
//     use ui::UI;
//     // use ffi_utils;
//     use std::os::raw::c_char;
//     use std::ffi::{CStr, CString};
//     use std::mem;
//     use libui_ffi::{self, uiDrawTextFont, uiDrawTextFontDescriptor, uiDrawTextLayout};

//     pub use libui_ffi::uiDrawTextWeight as Weight;
//     pub use libui_ffi::uiDrawTextItalic as Italic;
//     pub use libui_ffi::uiDrawTextStretch as Stretch;
//     pub use libui_ffi::uiDrawTextFontMetrics as FontMetrics;

//     pub struct FontDescriptor {
//         family: CString,
//         pub size: f64,
//         pub weight: Weight,
//         pub italic: Italic,
//         pub stretch: Stretch,
//     }

//     impl FontDescriptor {

//         pub fn new(
//             
//             family: &str,
//             size: f64,
//             weight: Weight,
//             italic: Italic,
//             stretch: Stretch,
//         ) -> FontDescriptor {
//             FontDescriptor {
//                 family: CString::new(family.as_bytes().to_vec()).unwrap(),
//                 size: size,
//                 weight: weight,
//                 italic: italic,
//                 stretch: stretch,
//             }
//         }

//         /// FIXME(pcwalton): Should this return an Option?

//         pub fn load_closest_font(&self) -> Font {
//             unsafe {
//                 let font_descriptor = uiDrawTextFontDescriptor {
//                     Family: self.family.as_ptr(),
//                     Size: self.size,
//                     Weight: self.weight,
//                     Italic: self.italic,
//                     Stretch: self.stretch,
//                 };
//                 Font {
//                     ui_draw_text_font: libui_ffi::uiDrawLoadClosestFont(&font_descriptor),
//                 }
//             }
//         }

//         pub fn family(&self) -> &str {
//             self.family.to_str().unwrap()
//         }
//     }

//     pub struct Font {
//         ui_draw_text_font: *mut uiDrawTextFont,
//     }

//     impl Drop for Font {

//         fn drop(&mut self) {
//             unsafe { libui_ffi::uiDrawFreeTextFont(self.ui_draw_text_font) }
//         }
//     }

//     impl Font {

//         pub unsafe fn from_ui_draw_text_font(ui_draw_text_font: *mut uiDrawTextFont) -> Font {
//             Font {
//                 ui_draw_text_font: ui_draw_text_font,
//             }
//         }

//         pub fn handle(&self) -> usize {
//             unsafe { libui_ffi::uiDrawTextFontHandle(self.ui_draw_text_font) }
//         }

//         pub fn describe(&self) -> FontDescriptor {
//             unsafe {
//                 let mut ui_draw_text_font_descriptor = mem::uninitialized();
//                 libui_ffi::uiDrawTextFontDescribe(
//                     self.ui_draw_text_font,
//                     &mut ui_draw_text_font_descriptor,
//                 );
//                 let family = CStr::from_ptr(ui_draw_text_font_descriptor.Family)
//                     .to_bytes()
//                     .to_vec();
//                 let font_descriptor = FontDescriptor {
//                     family: CString::new(family).unwrap(),
//                     size: ui_draw_text_font_descriptor.Size,
//                     weight: ui_draw_text_font_descriptor.Weight,
//                     italic: ui_draw_text_font_descriptor.Italic,
//                     stretch: ui_draw_text_font_descriptor.Stretch,
//                 };
//                 libui_ffi::uiFreeText(ui_draw_text_font_descriptor.Family as *mut c_char);
//                 font_descriptor
//             }
//         }

//         pub fn metrics(&self) -> FontMetrics {
//             unsafe {
//                 let mut metrics = mem::uninitialized();
//                 libui_ffi::uiDrawTextFontGetMetrics(self.ui_draw_text_font, &mut metrics);
//                 metrics
//             }
//         }
//     }

//     pub struct Layout {
//         ui_draw_text_layout: *mut uiDrawTextLayout,
//     }

//     impl Drop for Layout {

//         fn drop(&mut self) {
//             unsafe { libui_ffi::uiDrawFreeTextLayout(self.ui_draw_text_layout) }
//         }
//     }

//     impl Layout {

//         pub fn new( text: &str, default_font: &Font, width: f64) -> Layout {
//             unsafe {
//                 let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
//                 Layout {
//                     ui_draw_text_layout: libui_ffi::uiDrawNewTextLayout(
//                         c_string.as_ptr(),
//                         default_font.ui_draw_text_font,
//                         width,
//                     ),
//                 }
//             }
//         }

//         pub fn as_ui_draw_text_layout(&self) -> *mut uiDrawTextLayout {
//             self.ui_draw_text_layout
//         }

//         pub fn set_width(&self,  width: f64) {
//             unsafe { libui_ffi::uiDrawTextLayoutSetWidth(self.ui_draw_text_layout, width) }
//         }

//         pub fn extents(&self) -> (f64, f64) {
//             unsafe {
//                 let mut extents = (0.0, 0.0);
//                 libui_ffi::uiDrawTextLayoutExtents(
//                     self.ui_draw_text_layout,
//                     &mut extents.0,
//                     &mut extents.1,
//                 );
//                 extents
//             }
//         }

//         pub fn set_color(&self,  start_char: i64, end_char: i64, r: f64, g: f64, b: f64, a: f64) {
//             unsafe {
//                 libui_ffi::uiDrawTextLayoutSetColor(
//                     self.ui_draw_text_layout,
//                     start_char,
//                     end_char,
//                     r,
//                     g,
//                     b,
//                     a,
//                 )
//             }
//         }
//     }
// }
