#![recursion_limit = "128"]

use log::info;
use rand::seq::SliceRandom;
use yew::html;
use yew::prelude::*;
use yew_table::{columns, Table};
use rand::Rng;
use strum::IntoEnumIterator;
use yew_app::task::*;
use chrono::prelude::*;

struct Model {
    tasks: Vec<Task>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut rng = rand::thread_rng();
        let task_statuses = TaskStatus::iter().collect::<Vec<_>>();;
        let tasks: Vec<Task> = (0..100)
            .map(|i| Task {
                id: format!("task-{}", i + 1),
                description: String::from("These are not the Lorem Ipsums you are looking for"),
                due_date: Some(Utc.ymd(2014, (i % 12) + 1, 8).and_hms(12, 0, 9)),
                status: task_statuses.choose(&mut rng).unwrap().to_owned(),
                is_favorite: rng.gen(),
                is_archived: rng.gen(),
                ..Task::default()
            })
            .collect();

        Model { tasks }
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
            ("is_favorite", "Fav.")
            ("is_archived", "Arch.")
        ];

        html! {
            <>
                <Table<Task>: columns=columns, data=&self.tasks,/>
            </>
        }
    }
}

fn main() {
    web_logger::init();
    info!("Starting SPA");
    yew::start_app::<Model>();
}
