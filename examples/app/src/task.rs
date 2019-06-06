use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum_macros::EnumIter;
use std::fmt;
use yew::html;
use yew::prelude::*;
use yew_table::{Table, TableData};

#[derive(SmartDefault, Clone, PartialEq, Debug, Serialize, Deserialize, EnumIter)]
pub enum TaskStatus {
    #[default]
    Open,
    Paused,
    Closed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
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
    fn get_field_as_html(&self, field_name: &str) -> Html<Table<Self>> {
        match field_name {
            "id" => html! {
                { &self.id }
            },
            "description" => html! {
                { self.description.clone() }
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
            n => panic!("HTML representation of field name '{}' does not exist", n),
        }
    }
}