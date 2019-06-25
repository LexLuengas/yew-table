use yew::Html;
use serde::{Serialize};
use serde_value::Value;
use std::fmt;
use smart_default::SmartDefault;
use crate::error::*;

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html<Table<Self>>>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

#[derive(Clone, PartialEq, SmartDefault)]
pub struct TableOptions {
    pub orderable: bool,
}

#[derive(Copy, Clone, PartialEq, SmartDefault)]
pub enum TableOrder {
    #[default]
    Unordered = 0,
    Ascending = 1,
    Descending = 2,
}

impl TableOrder {
    pub fn rotate(&self) -> Self {
        use TableOrder::*;
        match *self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct TableState {
    pub order: Vec<TableOrder>,
}

/// The a table with columns holding data.
#[derive(Clone, PartialEq, Default)]
pub struct Table<T> where T: TableData {
    /// The order of the columns determines the order in which they are displayed.
    pub (crate) columns: Vec<Column>,
    pub (crate) data: Vec<T>,
    pub (crate) options: Option<TableOptions>,
    pub (crate) state: TableState,
}

impl<T> Table<T> where T: TableData {
    pub fn is_orderable(&self) -> bool {
        if let Some(options) = &self.options {
            options.orderable
        } else {
            false
        }
    }
}