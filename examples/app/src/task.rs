use chrono::prelude::*;
use serde::Serialize;
use serde_value::Value;
use smart_default::SmartDefault;
use strum_macros::{EnumIter, ToString};
use yew::html;
use yew::prelude::*;
use yew_table::{Table, TableData, TableError, Result as TableResult};

#[derive(
    Clone, PartialOrd, Eq, PartialEq, Ord, 
    Serialize, EnumIter, ToString, SmartDefault)]
#[strum(serialize_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Open = 0,
    Paused = 1,
    Closed = 2,
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub responsible_user: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub progress: i8,
    pub is_locked: bool,
    pub is_archived: bool,
    pub is_favorite: bool,
}

impl TableData for Task {
    fn get_field_as_html(&self, field_name: &str) -> TableResult<Html<Table<Self>>> {
        let html_repr = match field_name {
            "id" => html! {
                { &self.id }
            },
            "description" => html! {
                { &self.description }
            },
            "status" => html! {
                { self.status.to_string() }
            },
            "due_date" => html! {
                { self.due_date
                    .map_or(
                        String::default(),
                        |dt| dt.format("%c").to_string()
                    ) }
            },
            "progress" => html! {
                { self.progress.to_string() + "%" }
            },
            "is_favorite" => html! {
                <input type="checkbox", checked=self.is_favorite,/>
            },
            "is_archived" => html! {
                <input type="checkbox", checked=self.is_archived,/>
            },
            n => return Err(TableError::NonRenderableField(n.to_owned())),
        };
        Ok(html_repr)
    }

    fn get_field_as_value(&self, field_name: &str) -> TableResult<Value> {
        let value = match field_name {
            "id" => serde_value::to_value(
                &self.id
                    .chars().skip(5).collect::<String>() // omit prefix "task-"
                    .parse::<i32>().unwrap() // parse the number as integer
            ),
            "description" => serde_value::to_value(&self.description),
            "status" => serde_value::to_value(self.status.to_string()),
            "due_date" => serde_value::to_value(self.due_date.map_or(0, |d| d.timestamp())),
            "progress" => serde_value::to_value(self.progress),
            "is_favorite" => serde_value::to_value(self.is_favorite),
            "is_archived" => serde_value::to_value(self.is_archived),
            n => return Err(TableError::InvalidFieldName(n.to_owned())),
        };
        Ok(value.unwrap())
    }
}
