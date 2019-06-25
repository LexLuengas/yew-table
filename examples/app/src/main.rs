#![recursion_limit = "128"]

use log::info;
use rand::seq::SliceRandom;
use yew::html;
use yew::prelude::*;
use yew_table::{columns, Table, TableOptions};
use rand::Rng;
use strum::IntoEnumIterator;
use yew_app::task::*;
use chrono::prelude::*;

struct Model {
    tasks: Vec<Task>,
}

fn create_mock_tasks() -> Vec<Task> {
    let mut rng = rand::thread_rng();
    let task_statuses = TaskStatus::iter().collect::<Vec<_>>();;
    (0..100)
        .map(|i| Task {
            id: format!("task-{}", i + 1),
            description: String::from("These are not the Lorem Ipsums you are looking for"),
            due_date: Some(Utc.ymd(2014, (i % 12) + 1, 8).and_hms(12, 0, 9)),
            status: task_statuses.choose(&mut rng).unwrap().to_owned(),
            is_favorite: rng.gen(),
            is_archived: rng.gen(),
            ..Task::default()
        })
        .collect()
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            tasks: create_mock_tasks()
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let columns = columns![
            ("id", "Id.")
            ("description", "Description")
            ("due_date", "Due date")
            ("status", "Status")
            ("is_favorite", "Favorite", "Fav.")
            ("is_archived", "Archived", "Arch.")
        ];

        let options = TableOptions {
            orderable: true,
        };

        html! {
            <>
                <Table<Task>: columns=columns, data=&self.tasks, options=Some(options),/>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    info!("Starting SPA");
    yew::start_app::<Model>();
}
