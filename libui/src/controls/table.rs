use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_int, c_void};
use std::rc::Rc;
use libui_ffi::{
    self, uiControl, uiTable, uiTableModel, uiTableModelHandler, uiTableParams, uiTableValue,
    uiTableValueType,
};

/// An enum of possible `Table` cell/column types.
#[derive(Copy, Clone, Debug)]
pub enum TableValueType {
    String,
    Image,
    Int,
    Color,
}

/// An enum representing the value of a `Table` cell.
pub enum TableValue {
    Int(i32),
    String(String),
    Color { r: f64, g: f64, b: f64, a: f64 },
}

pub trait TableDataSource {
    fn num_columns(&mut self) -> i32;
    fn num_rows(&mut self) -> i32;
    fn column_type(&mut self, column: i32) -> TableValueType;

    fn cell(&mut self, column: i32, row: i32) -> TableValue;
    fn set_cell(&mut self, column: i32, row: i32, value: TableValue);
}

extern "C" fn c_num_columns(
    ui_handler: *mut uiTableModelHandler,
    _ui_model: *mut uiTableModel,
) -> c_int {
    unsafe {
        // This cast is safe because RustTableModelHandler has a compatible layout.
        // Unfortunately we can't do the same with `ui_model` because we don't store
        // the object itself but just a pointer we didn't create.
        // TODO: deal with the model somehow.
        (*(ui_handler as *mut RustTableModelHandler))
            .trait_object
            .borrow_mut()
            .num_columns()
    }
}

extern "C" fn c_num_rows(
    ui_handler: *mut uiTableModelHandler,
    _ui_model: *mut uiTableModel,
) -> c_int {
    unsafe {
        (*(ui_handler as *mut RustTableModelHandler))
            .trait_object
            .borrow_mut()
            .num_rows()
    }
}

extern "C" fn c_column_type(
    ui_handler: *mut uiTableModelHandler,
    _ui_model: *mut uiTableModel,
    column: c_int,
) -> uiTableValueType {
    let t = unsafe {
        (*(ui_handler as *mut RustTableModelHandler))
            .trait_object
            .borrow_mut()
            .column_type(column)
    };

    match t {
        TableValueType::String => libui_ffi::uiTableValueTypeString,
        TableValueType::Image => libui_ffi::uiTableValueTypeImage,
        TableValueType::Int => libui_ffi::uiTableValueTypeInt,
        TableValueType::Color => libui_ffi::uiTableValueTypeColor,
    }
}

extern "C" fn c_cell_value(
    ui_handler: *mut uiTableModelHandler,
    _ui_model: *mut uiTableModel,
    row: c_int,
    column: c_int,
) -> *mut uiTableValue {
    let value = unsafe {
        (*(ui_handler as *mut RustTableModelHandler))
            .trait_object
            .borrow_mut()
            .cell(column, row)
    };

    match value {
        TableValue::Int(v) => unsafe { libui_ffi::uiNewTableValueInt(v) },
        TableValue::String(s) => unsafe {
            let c_string = CString::new(s.as_bytes().to_vec()).unwrap();
            libui_ffi::uiNewTableValueString(c_string.as_ptr())
        },
        TableValue::Color { r, g, b, a } => unsafe { libui_ffi::uiNewTableValueColor(r, g, b, a) },
    }
}

extern "C" fn c_set_cell_value(
    ui_handler: *mut uiTableModelHandler,
    _ui_model: *mut uiTableModel,
    row: c_int,
    column: c_int,
    value: *const uiTableValue,
) {
    unsafe {
        // Button columns call SetCellValue() with a value of `NULL` in case of
        // a click. We don't want to have a special enum or Option<TableValue> just for
        // this one instance, so we provide an integer instead. The function is only ever
        // called for clicks anyway, making a check on the users side unnecessary.
        if value == std::ptr::null() {
            (*(ui_handler as *mut RustTableModelHandler))
                .trait_object
                .borrow_mut()
                .set_cell(column, row, TableValue::Int(0));
            return;
        }

        let vt = libui_ffi::uiTableValueGetType(value);

        let rust_value = match vt {
            libui_ffi::uiTableValueTypeInt => {
                let i = libui_ffi::uiTableValueInt(value);
                TableValue::Int(i)
            }
            libui_ffi::uiTableValueTypeString => {
                let s = libui_ffi::uiTableValueString(value);
                TableValue::String(CStr::from_ptr(s).to_string_lossy().into_owned())
            }
            libui_ffi::uiTableValueTypeColor => {
                let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
                libui_ffi::uiTableValueColor(value, &mut r, &mut g, &mut b, &mut a);
                TableValue::Color { r, g, b, a }
            }
            _ => panic!("Unsupported table value type"),
        };

        (*(ui_handler as *mut RustTableModelHandler))
            .trait_object
            .borrow_mut()
            .set_cell(column, row, rust_value);
    }
}

#[repr(C)]
struct RustTableModelHandler {
    ui_table_model_handler: uiTableModelHandler,
    trait_object: Rc<RefCell<dyn TableDataSource>>,
}

impl RustTableModelHandler {
    fn new(trait_object: Rc<RefCell<dyn TableDataSource>>) -> Self {
        RustTableModelHandler {
            ui_table_model_handler: libui_ffi::uiTableModelHandler {
                NumColumns: Some(c_num_columns),
                ColumnType: Some(c_column_type),
                NumRows: Some(c_num_rows),
                CellValue: Some(c_cell_value),
                SetCellValue: Some(c_set_cell_value),
            },
            trait_object,
        }
    }
}

/// The view model for a `Table` control.
pub struct TableModel {
    ui_table_model: *mut libui_ffi::uiTableModel,
    _model_handler: Box<RustTableModelHandler>,
}

impl TableModel {
    pub fn new(data_source: Rc<RefCell<dyn TableDataSource>>) -> TableModel {
        unsafe {
            let mut handler = Box::new(RustTableModelHandler::new(data_source));
            let ptr = handler.as_mut() as *mut RustTableModelHandler;
            TableModel {
                ui_table_model: libui_ffi::uiNewTableModel(ptr as *mut libui_ffi::uiTableModelHandler),
                _model_handler: handler, // We store the object to bind its lifetime to ours.
            }
        }
    }

    /// Informs all associated `Table` views that a new row has been added.
    ///
    /// You must insert the row data in your model before calling this function.
    /// `TableDataSource::num_rows()` must represent the new row count before you call this function.
    pub fn notify_row_inserted(&self, new_row: i32) {
        unsafe {
            libui_ffi::uiTableModelRowInserted(self.ui_table_model, new_row);
        }
    }

    /// Informs all associated `Table` views that a row has been changed.
    ///
    /// You do NOT need to call this method from `TableDataSource::set_cell()`, but NEED
    /// to call this if your data changes at any other point.
    pub fn notify_row_changed(&self, row: i32) {
        unsafe {
            libui_ffi::uiTableModelRowChanged(self.ui_table_model, row);
        }
    }

    /// Informs all associated `Table` views that a row has been deleted.
    ///
    /// You must delete the row from your model before you call this function.
    /// `TableDataSource::num_rows()` must represent the new row count before you call this function.
    pub fn notify_row_deleted(&self, old_row: i32) {
        unsafe {
            libui_ffi::uiTableModelRowDeleted(self.ui_table_model, old_row);
        }
    }
}

impl Drop for TableModel {
    fn drop(&mut self) {
        unsafe {
            libui_ffi::uiFreeTableModel(self.ui_table_model);
        }
    }
}

/// The parameters to construct a `Table` with.
pub struct TableParameters {
    model: Rc<RefCell<TableModel>>,
    row_background_color_column: i32,
}

impl TableParameters {
    pub fn new(model: Rc<RefCell<TableModel>>) -> TableParameters {
        TableParameters {
            model: model,
            row_background_color_column: -1,
        }
    }
}

/// Describes a visual sorting indicator for `Table` columns.
#[derive(Copy, Clone, Debug)]
pub enum SortIndicator {
    None,
    Ascending,
    Descending,
}

/// Describes how many `Table` rows can be selected.
#[derive(Copy, Clone, Debug)]
pub enum SelectionMode {
    None,
    ZeroOrOne,
    One,
    ZeroOrMany,
}

/// A structure holding additional information on text columns.
#[derive(Copy, Clone, Debug)]
pub struct TextColumnParameters {
    pub text_color_column: i32,
}

impl Default for TextColumnParameters {
    fn default() -> Self {
        Self {
            text_color_column: -1,
        }
    }
}

define_control! {
    /// A tabular control that can be used to display and edit data.
    /// The table itself does not store any data but is a "View" on
    /// a [`TableModel`] / [`TableDataSource`] which update the table
    /// and provide the data. It follows the the concept of separation of
    /// concerns, similar to common patterns like model-view-controller or
    /// model-view-adapter.
    ///
    /// Users must implement the [`TableDataSource`] trait to serve as their data storage
    /// or data storage adapter. It provides the actual data while also handling data edits.
    ///
    /// Then a [`TableModel`] object can be created from it. `TableModel` acts as a delegate
    /// for the underlying data store. Its purpose is to provide the data for views and inform
    /// about any updates.
    ///
    /// With the model, a new table can be created via [`Table::new()`]. Use the `append_XXX()`
    /// methods of the table object to select which data is to be displayed and how.
    rust_type: Table,
    sys_type: uiTable
}

impl Table {
    pub const COLUMN_READONLY: i32 = -1;
    pub const COLUMN_EDITABLE: i32 = -2;

    /// Instantiates a new `Table` using the supplied model.
    pub fn new(params: TableParameters) -> Table {
        unsafe {
            let mut ui_params = uiTableParams {
                Model: params.model.borrow().ui_table_model,
                RowBackgroundColorModelColumn: params.row_background_color_column,
            };
            // TODO: keep the model alive via the shared ptr. This requires storing it tho.
            // Because i am lazy, i leak the model for now.
            mem::forget(params.model);
            Table {
                // The parameter struct is not stored. we can safely provide
                // a raw pointer and let the struct go out of scope. Only the
                // uiTableModel inside must be kept alive.
                uiTable: libui_ffi::uiNewTable(&mut ui_params as *mut uiTableParams),
            }
        }
    }

    /// Appends a text column to the table.
    ///
    /// * `title`               - The columns header.
    /// * `text_model_column`   - Index to the model column with the text data ([`TableValue::String`]).
    /// * `state_model_column`  - Index to the model column with the state data ([`TableValue::Int`]). An entry with value != `0`
    ///                           means the text shall be editable. Alternatively use [`Table::COLUMN_EDITABLE`] or [`Table::COLUMN_READONLY`]
    ///                           for this parameter to make all rows either state.
    pub fn append_text_column(
        &mut self,

        title: &str,
        text_model_column: i32,
        state_model_column: i32,
    ) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            libui_ffi::uiTableAppendTextColumn(
                self.uiTable,
                c_title.as_ptr(),
                text_model_column,
                state_model_column,
                std::ptr::null_mut(), // TODO: support text params
            );
        }
    }

    /// Appends a text column to the table, allowing for colored text using the [`TextColumnParameters`] argument.
    ///
    /// * `title`               - The columns header.
    /// * `text_model_column`   - Index to the model column with the text data ([`TableValue::String`]),
    /// * `state_model_column`  - Index to the model column with the state data ([`TableValue::Int`]). An entry with value != `0`
    ///                           means the text shall be editable. Alternatively use [`Table::COLUMN_EDITABLE`] or [`Table::COLUMN_READONLY`]
    ///                           for this parameter to make all rows either state.
    /// * `params`              - [`TextColumnParameters::text_color_column`] must point to a [`TableValue::Color`] column in
    ///                           the table model, or be `-1` for using the default text color.
    pub fn append_text_column_with_params(
        &mut self,
        title: &str,
        text_model_column: i32,
        state_model_column: i32,
        params: TextColumnParameters,
    ) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            let mut c_params = libui_ffi::uiTableTextColumnOptionalParams {
                ColorModelColumn: params.text_color_column,
            };
            libui_ffi::uiTableAppendTextColumn(
                self.uiTable,
                c_title.as_ptr(),
                text_model_column,
                state_model_column,
                &mut c_params as *mut libui_ffi::uiTableTextColumnOptionalParams,
            );
        }
    }

    // TODO: uiTableAppendImageColumn
    // TODO: uiTableAppendImageTextColumn

    /// Appends a column to the table containing a checkbox.
    ///
    /// * `title`               - The columns header.
    /// * `check_model_column`  - Index to the model column with the checkbox data.
    ///                           Must be of [`TableValue::Int`], where a value of `0` is an unchecked box.
    ///                           A value not equal to `0` results in a checked box.
    /// * `state_model_column`  - Index to the model column with the state data ([`TableValue::Int`]). An entry with value != `0`
    ///                           means the checkbox shall be editable. Alternatively use [`Table::COLUMN_EDITABLE`] or [`Table::COLUMN_READONLY`]
    ///                           for this parameter to make all rows either state.
    pub fn append_checkbox_column(
        &mut self,
        title: &str,
        check_model_column: i32,
        state_model_column: i32,
    ) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            libui_ffi::uiTableAppendCheckboxColumn(
                self.uiTable,
                c_title.as_ptr(),
                check_model_column,
                state_model_column,
            );
        }
    }

    /// Appends a column to the table containing a checkbox and text.
    ///
    /// * `title`                   - The columns header.
    /// * `check_model_column`      - Index to the model column with the checkbox data.
    ///                               Must be of [`TableValue::Int`], where a value of `0` is an unchecked box.
    ///                               A value not equal to `0` results in a checked box.
    /// * `state_model_column`      - Index to the model column with the state data ([`TableValue::Int`]). An entry with value != `0`
    ///                               means the checkbox shall be editable. Alternatively use [`Table::COLUMN_EDITABLE`] or [`Table::COLUMN_READONLY`]
    ///                               for this parameter to make all rows either state.
    /// * `text_model_column`       - Index to the model column with the text data. Must be [`TableValue::String`].
    /// * `text_state_model_colum`  - Defines whether or not the text is editable, analogous to `state_model_column`.
    pub fn append_checkbox_text_column(
        &mut self,
        title: &str,
        check_model_column: i32,
        state_model_column: i32,
        text_model_column: i32,
        text_state_model_colum: i32,
    ) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            libui_ffi::uiTableAppendCheckboxTextColumn(
                self.uiTable,
                c_title.as_ptr(),
                check_model_column,
                state_model_column,
                text_model_column,
                text_state_model_colum,
                std::ptr::null_mut(),
            );
        }
    }

    /// Appends a column to the table containing a progress bar.
    ///
    /// * `title`           - The columns header.
    /// * `model_column`    - Index to the model column with the progessbar data.
    ///                       Values must be of [`TableValue::Int`], between -1 and 100 representing the current progress.
    pub fn append_progressbar_column(&mut self, title: &str, model_column: i32) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            libui_ffi::uiTableAppendProgressBarColumn(self.uiTable, c_title.as_ptr(), model_column);
        }
    }

    /// Appends a column to the table containing a button.
    ///
    /// * `title`               - The columns header.
    /// * `btn_model_column`    - Index to the model column with the button text ([`TableValue::String`]).
    ///                           Clicks are signaled to the [`TableDataSource`] by calling [`TableDataSource::set_cell()`]
    ///                           with a `TableValue::Int()`. Because no other reason for the call exists, checking the type is not strictly necessary.
    /// * `state_model_column`  - Index to the model column with the state data ([`TableValue::Int`]). An entry with value != `0`
    ///                           means the button shall be clickable. Alternatively use [`Table::COLUMN_EDITABLE`] or [`Table::COLUMN_READONLY`]
    ///                           for this parameter to make all rows either state.
    pub fn append_button_column(
        &mut self,
        title: &str,
        btn_model_column: i32,
        state_model_column: i32,
    ) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            libui_ffi::uiTableAppendButtonColumn(
                self.uiTable,
                c_title.as_ptr(),
                btn_model_column,
                state_model_column,
            );
        }
    }

    /// Returns whether or not the table header is visible.
    pub fn header_visible(&self) -> bool {
        unsafe { libui_ffi::uiTableHeaderVisible(self.uiTable) != 0 }
    }

    /// Sets whether or not the table header is visible.
    pub fn set_header_visible(&mut self, visible: bool) {
        unsafe {
            libui_ffi::uiTableHeaderSetVisible(self.uiTable, visible as i32);
        }
    }

    /// Returns the column's sort indicator displayed in the table header.
    pub fn sort_indicator(&self, column: i32) -> SortIndicator {
        let v = unsafe { libui_ffi::uiTableHeaderSortIndicator(self.uiTable, column) };
        match v {
            libui_ffi::uiSortIndicatorNone => SortIndicator::None,
            libui_ffi::uiSortIndicatorAscending => SortIndicator::Ascending,
            libui_ffi::uiSortIndicatorDescending => SortIndicator::Descending,
            _ => panic!("Invalid sort indicator value"),
        }
    }

    /// Sets the column's sort indicator displayed in the table header.
    ///
    /// Use this to display appropriate arrows in the table header to indicate a sort direction.
    /// Setting the indicator is purely visual and does not perform any sorting.
    pub fn set_sort_indicator(&mut self, column: i32, indicator: SortIndicator) {
        let v = match indicator {
            SortIndicator::None => libui_ffi::uiSortIndicatorNone,
            SortIndicator::Ascending => libui_ffi::uiSortIndicatorAscending,
            SortIndicator::Descending => libui_ffi::uiSortIndicatorDescending,
        };
        unsafe {
            libui_ffi::uiTableHeaderSetSortIndicator(self.uiTable, column, v);
        }
    }

    /// Returns the table column width in pixels.
    pub fn column_width(&self, column: i32) -> i32 {
        unsafe { libui_ffi::uiTableColumnWidth(self.uiTable, column) }
    }

    /// Sets the table column width in pixels.
    ///
    /// Setting the width to `-1` will restore automatic column sizing, matching
    /// either the width of the content or column header (which ever one is bigger).
    /// Note: Mac OS only resizes to the column header, not the content.
    pub fn set_column_width(&mut self, column: i32, width: i32) {
        unsafe {
            libui_ffi::uiTableColumnSetWidth(self.uiTable, column, width);
        }
    }

    /// Returns the table selection mode.
    pub fn selection_mode(&self) -> SelectionMode {
        let v = unsafe { libui_ffi::uiTableGetSelectionMode(self.uiTable) };
        match v {
            libui_ffi::uiTableSelectionModeNone => SelectionMode::None,
            libui_ffi::uiTableSelectionModeZeroOrOne => SelectionMode::ZeroOrOne,
            libui_ffi::uiTableSelectionModeOne => SelectionMode::One,
            libui_ffi::uiTableSelectionModeZeroOrMany => SelectionMode::ZeroOrMany,
            _ => panic!("Invalid selection mode value"),
        }
    }

    /// Sets the table selection mode.
    ///
    /// Note: All rows will be deselected if the existing selection is illegal in the new selection mode.
    pub fn set_selection_mode(&mut self, mode: SelectionMode) {
        let v = match mode {
            SelectionMode::None => libui_ffi::uiTableSelectionModeNone,
            SelectionMode::ZeroOrOne => libui_ffi::uiTableSelectionModeZeroOrOne,
            SelectionMode::One => libui_ffi::uiTableSelectionModeOne,
            SelectionMode::ZeroOrMany => libui_ffi::uiTableSelectionModeZeroOrMany,
        };
        unsafe {
            libui_ffi::uiTableSetSelectionMode(self.uiTable, v);
        }
    }

    /// Returns the current table selection.
    ///
    /// If nothing is selected, the vector will be empty.
    pub fn selection(&self) -> Vec<i32> {
        let mut selection: Vec<i32> = vec![];
        unsafe {
            let s = libui_ffi::uiTableGetSelection(self.uiTable);
            let p = (*s).Rows;
            for i in 0..(*s).NumRows {
                let v = *(p.offset(i as isize));
                selection.push(v);
            }
            libui_ffi::uiFreeTableSelection(s);
        }
        selection
    }

    /// Sets the current table selection clearing any previous selection.
    ///
    /// Selecting more rows than the selection mode allows for results in nothing happening.
    pub fn set_selection(&mut self, selection: &Vec<i32>) {
        unsafe {
            // Our vector is only read, despite what struct and func signature say
            let mut s = libui_ffi::uiTableSelection {
                NumRows: selection.len() as i32,
                Rows: selection.as_ptr() as *mut i32,
            };
            libui_ffi::uiTableSetSelection(self.uiTable, &mut s as *mut libui_ffi::uiTableSelection);
        }
    }

    /// Registers a callback for when the table selection changed.
    ///
    /// Note: The callback is not triggered when calling `set_selection()` or when
    /// the selection is cleared due to `set_selection_mode()`.
    pub fn on_selection_changed<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(&mut Table) + 'static,
    {
        extern "C" fn c_callback<G>(table: *mut uiTable, data: *mut c_void)
        where
            G: FnMut(&mut Table),
        {
            let mut table = Table { uiTable: table };
            unsafe {
                from_void_ptr::<G>(data)(&mut table);
            }
        }
        unsafe {
            libui_ffi::uiTableOnSelectionChanged(
                self.uiTable,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }

    extern "C" fn generic_table_callback<G>(
        table: *mut uiTable,
        row_or_column: i32,
        data: *mut c_void,
    ) where
        G: FnMut(&mut Table, i32),
    {
        let mut table = Table { uiTable: table };
        unsafe {
            from_void_ptr::<G>(data)(&mut table, row_or_column);
        }
    }

    /// Registers a callback for when the user single clicks a table row.
    ///
    /// Note: Only one callback can be registered at a time.
    pub fn on_row_clicked<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(&mut Table, i32) + 'static,
    {
        unsafe {
            libui_ffi::uiTableOnRowClicked(
                self.uiTable,
                Some(Self::generic_table_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }

    /// Registers a callback for when the user double clicks a table row.
    ///
    /// Note: Only one callback can be registered at a time.
    /// Bug: The double click callback is always preceded by one `on_row_clicked()` callback.
    /// For unix systems linking against `GTK < 3.14` the preceding `on_row_clicked()` callback
    /// will be triggered twice.
    pub fn on_row_double_clicked<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(&mut Table, i32) + 'static,
    {
        unsafe {
            libui_ffi::uiTableOnRowDoubleClicked(
                self.uiTable,
                Some(Self::generic_table_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }

    /// Registers a callback for when a table column header is clicked.
    ///
    /// Note: Only one callback can be registered at a time.
    pub fn on_header_clicked<'ctx, F>(&mut self, callback: F)
    where
        F: FnMut(&mut Table, i32) + 'static,
    {
        unsafe {
            libui_ffi::uiTableHeaderOnClicked(
                self.uiTable,
                Some(Self::generic_table_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
