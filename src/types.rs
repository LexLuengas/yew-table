//! Types necessary for yew-table
use super::error::Result;
use serde::Serialize;
use serde_value::Value;
use std::fmt;
use yew::Html;

/// Trait that handles data display and sorting for the table
pub trait TableData: 'static + Default + Clone + Ord + Serialize {
    /// Returns the Html representation of a field. When None, the field is not rendered.
    fn get_field_as_html(&self, field_name: &str) -> Result<Html>;

    /// Returns a table value given its field name. This value is used as a sorting key for the corresponding column.
    fn get_field_as_value(&self, field_name: &str) -> Result<Value>;

    /// Returns true if table row matches search parameter
    fn matches_search(&self, needle: Option<String>) -> bool;
}

/// Struct for the table column
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Column {
    /// Column name, this is used instead of short_name for display if short_name is not set
    pub name: String,
    /// A value that is displayed in the table header
    pub short_name: Option<String>,
    /// This is needed to match data to the column
    pub data_property: Option<String>,
    /// Is this colum orderable?
    pub orderable: bool,
    /// Additional classes for the column header
    pub header_classes: Vec<String>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_name.as_ref().unwrap_or(&self.name))
    }
}

/// Builder for [Column]
#[derive(Default)]
pub struct ColumnBuilder {
    name: String,
    short_name: Option<String>,
    data_property: Option<String>,
    orderable: bool,
    header_classes: Vec<String>,
}

impl ColumnBuilder {
    /// Construct a new builder
    pub fn new(name: &str) -> ColumnBuilder {
        ColumnBuilder {
            name: name.to_string(),
            short_name: None,
            data_property: None,
            orderable: false,
            header_classes: vec![],
        }
    }

    /// Build a [Column] from the builder
    pub fn build(self) -> Column {
        Column {
            name: self.name,
            short_name: self.short_name,
            data_property: self.data_property,
            orderable: self.orderable,
            header_classes: self.header_classes,
        }
    }

    /// Sets the column as orderable
    pub fn orderable(mut self, orderable: bool) -> ColumnBuilder {
        self.orderable = orderable;
        self
    }

    /// Sets data property name
    pub fn data_property(mut self, data_property: &str) -> ColumnBuilder {
        self.data_property = Some(data_property.to_string());
        self
    }

    /// Sets column short name
    pub fn short_name(mut self, short_name: &str) -> ColumnBuilder {
        self.short_name = Some(short_name.to_string());
        self
    }

    /// Adds additional header class
    pub fn header_class(mut self, class: &str) -> ColumnBuilder {
        self.header_classes.push(class.to_string());
        self
    }
}

/// Column order
#[allow(missing_docs)]
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
    /// Rotate column order
    pub const fn rotate(self) -> Self {
        use TableOrder::{Ascending, Descending, Unordered};
        match self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}

/// Table order state
#[allow(missing_docs)]
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
    /// Table data
    pub data: Vec<T>,
    pub(crate) state: TableState,
}
