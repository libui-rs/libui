use draw::{Brush, Path, StrokeParams, Transform};
use libui_ffi::{self, uiDrawContext};

/// Drawing context, used to draw custom content on the screen.
pub struct DrawContext {
    ui_draw_context: *mut uiDrawContext,
}

impl DrawContext {
    /// Create a Context from a ui_draw_context pointer.
    ///
    /// # Unsafety
    /// If the pointer is invalid, this is memory-unsafe.
    /// If libui is not initialized, behavior will be inconsistent.
    pub unsafe fn from_ui_draw_context(ui_draw_context: *mut uiDrawContext) -> DrawContext {
        DrawContext {
            ui_draw_context: ui_draw_context,
        }
    }

    /// Draw a stroke on this DrawContext which runs along the given Path, with the given Brush and StrokeParams.
    pub fn stroke(&self, path: &Path, brush: &Brush, stroke_params: &StrokeParams) {
        unsafe {
            let brush = brush.as_ui_draw_brush_ref(self);
            let stroke_params = stroke_params.as_stroke_params_ref(self);
            libui_ffi::uiDrawStroke(
                self.ui_draw_context,
                path.ptr(),
                brush.ptr(),
                stroke_params.ptr(),
            )
        }
    }

    /// Draw a fill on this DrawContext using the given Path using the given Brush.
    pub fn fill(&self, path: &Path, brush: &Brush) {
        unsafe {
            let brush = brush.as_ui_draw_brush_ref(self);
            libui_ffi::uiDrawFill(self.ui_draw_context, path.ptr(), brush.ptr())
        }
    }

    /// Transform this DrawContext by the given Transform.
    pub fn transform(&self, txform: &Transform) {
        unsafe { libui_ffi::uiDrawTransform(self.ui_draw_context, txform.ptr()) }
    }

    /// Open a modal allowing the user to save the contents of this DrawContext.
    pub fn save(&self) {
        unsafe { libui_ffi::uiDrawSave(self.ui_draw_context) }
    }

    /// Open a modal allowing the user to load the contents of a DrawContext onto this one.
    pub fn restore(&self) {
        unsafe { libui_ffi::uiDrawRestore(self.ui_draw_context) }
    }
}
