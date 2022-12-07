use super::error::Result;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::Html;

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    /// # Errors
    /// Fails if unknown field is provided
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    /// # Errors
    /// Fails if unknown field is provided
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;

    /// Returns true if a row matches search value(needle)
    fn matches_search(&self, needle: Option<String>) -> bool;
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
    pub orderable: bool,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TableOrder {
    Unordered = 0,
    Ascending,
    Descending,
}

impl Default for TableOrder {
    fn default() -> Self {
        Self::Unordered
    }
}

impl TableOrder {
    #[must_use]
    pub const fn rotate(self) -> Self {
        use TableOrder::{Ascending, Descending, Unordered};
        match self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Default)]
pub struct TableState {
    pub order: Vec<TableOrder>,
}

/// The a table with columns holding data.
#[derive(Clone, Eq, PartialEq, Default)]
pub struct Table<T>
    where
        T: TableData,
{
    /// The order of the columns determines the order in which they are displayed.
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    pub state: TableState,
    pub orderable: bool,
}
