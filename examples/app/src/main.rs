#![recursion_limit = "128"]

use log::info;
use yew::html;
use yew::prelude::*;
use yew_table::{columns, Table};
use yew_app::task::*;
use chrono::prelude::*;

struct Model {
    tasks: Vec<Task>,
}

fn create_mock_tasks() -> Vec<Task> {
    let mut favorite = false;
    let mut archived = true;
    (0..100).map(|i| {
        favorite = !favorite;
        archived = !archived;
        Task {
            id: format!("task-{}", i + 1),
            description: String::from("These are not the Lorem Ipsums you are looking for"),
            due_date: Some(Utc.with_ymd_and_hms(2014, (i % 12) + 1, 8, 12, 0, 9).unwrap()),
            status: TaskStatus::Open,
            is_favorite: favorite,
            is_archived: archived,
            ..Task::default()
        }
    }).collect()
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            tasks: create_mock_tasks()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let columns = columns![
            ("id", "Id.", "Id.", true)
            ("description", "Description", "Description", true)
            ("due_date", "Due date", "Due date", true)
            ("status", "Status", "Status", true)
            ("is_favorite", "Favorite", "Fav.", true)
            ("is_archived", "Archived", "Arch.", true)
        ];

        html! {
            <>
                <Table<Task> classes={classes!("yew-table")} {columns} data={self.tasks.clone()} orderable={true}/>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("Starting SPA");
    yew::Renderer::<Model>::new().render();
}
