extern crate iui;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

use iui::controls::*;
use iui::prelude::*;

struct MyDataSource {
    data: Vec<(i32, String, i32, i32, i32, String)>,
}

impl Default for MyDataSource {
    fn default() -> Self {
        let mut d: Vec<(i32, String, i32, i32, i32, String)> = vec![];
        d.push((1, "Apples".into(), 0xCC0000, 1, 73, "Order".into()));
        d.push((2, "Oranges".into(), 0xFFAA00, 1, 68, "Order".into()));
        d.push((3, "Bananas".into(), 0xDDFF00, 0, 0, "Order".into()));
        d.push((4, "Limes".into(), 0x32CD32, 1, 12, "Order".into()));
        d.push((5, "Blueberries".into(), 0x483D8B, 0, 0, "Order".into()));
        d.push((6, "Plums".into(), 0x663399, 1, 28, "Order".into()));
        Self { data: d }
    }
}

impl TableDataSource for MyDataSource {
    fn num_columns(&mut self) -> i32 {
        4
    }

    fn num_rows(&mut self) -> i32 {
        self.data.len() as i32
    }

    fn column_type(&mut self, column: i32) -> TableValueType {
        match column {
            0 => TableValueType::String,
            1 => TableValueType::String,
            2 => TableValueType::Color,
            3 => TableValueType::Int,
            4 => TableValueType::Int,
            5 => TableValueType::String,
            _ => TableValueType::Int,
        }
    }

    fn cell(&mut self, column: i32, row: i32) -> TableValue {
        match column {
            0 => TableValue::String(self.data[row as usize].0.to_string()),
            1 => TableValue::String(self.data[row as usize].1.clone()),
            2 => TableValue::Color {
                r: ((self.data[row as usize].2 & 0xFF0000) >> 16) as f64 / 255.0,
                g: ((self.data[row as usize].2 & 0x00FF00) >> 8) as f64 / 255.0,
                b: ((self.data[row as usize].2 & 0x0000FF) >> 0) as f64 / 255.0,
                a: 1.0,
            },
            3 => TableValue::Int(self.data[row as usize].3.clone()),
            4 => TableValue::Int(self.data[row as usize].4.clone()),
            5 => TableValue::String(self.data[row as usize].5.clone()),
            _ => TableValue::Int(0),
        }
    }

    fn set_cell(&mut self, _column: i32, _row: i32, _value: TableValue) {
        todo!()
    }
}

pub fn make_table_page(_ui: UI) -> Control {
    let mut vbox = VerticalBox::new();
    vbox.set_padded(true);

    let name_params = TextColumnParameters {
        text_color_column: 2,
    };

    let data = Rc::new(RefCell::new(MyDataSource::default()));
    let model = Rc::new(RefCell::new(TableModel::new(data.clone())));
    let params = TableParameters::new(model.clone());
    let mut table = Table::new(params);

    table.append_text_column("ID", 0, Table::COLUMN_READONLY);
    table.append_text_column_with_params("Name", 1, Table::COLUMN_READONLY, name_params);
    table.append_checkbox_column("Availability", 3, Table::COLUMN_READONLY);
    table.append_progressbar_column("Stock", 4);
    table.append_button_column("", 5, Table::COLUMN_EDITABLE);

    vbox.append(table, LayoutStrategy::Stretchy);
    return vbox.into();
}
