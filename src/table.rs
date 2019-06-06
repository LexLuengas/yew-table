use yew::prelude::*;
use yew::html;
use serde::{Serialize};

pub trait TableData: 'static + Default + Clone + PartialEq + Serialize {
    fn get_field_as_html(&self, field_name: &str) -> Html<Table<Self>>;
}
// impl<T: 'static + Default + Clone + PartialEq + Serialize> TableData for T {}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct Table<T> where T: TableData {
    columns: Vec<Column>,
    data: Vec<T>,
}

#[derive(Clone, PartialEq, Default)]
pub struct TableProps<T> where T: TableData {
    pub columns: Vec<Column>,
    pub data: Vec<T>,
}

impl<T> Component for Table<T> where T: TableData {
    type Message = ();
    type Properties = TableProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Table {
            columns: props.columns,
            data: props.data,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.columns = props.columns;
        self.data = props.data;
        true
    }
}

impl<T> Renderable<Table<T>> for Table<T> where T: TableData {
    fn view(&self) -> Html<Table<T>> {
        html! {
            <table>
                <thead>
                    { for self.columns.iter().map(Column::view) }
                </thead>
                <tbody>
                    { for self.data.iter().map(|d| self.view_row(&d)) }
                </tbody>
            </table>
        }
    }
}

impl<T> Renderable<Table<T>> for Column where T: TableData {
    fn view(&self) -> Html<Table<T>> {
        html! {
            <th><abbr title=&self.name,>{ self.short_name.as_ref().unwrap_or(&self.name) }</abbr></th>
        }
    }
}

impl<T> Table<T> where T: TableData {
    fn view_row(&self, datum: &T) -> Html<Table<T>> {
        html! {
            <tr>
                { 
                    for self.columns.iter()
                        .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                        .map(|name| { datum.get_field_as_html(name) })
                        .map(|el| html! { <td>{ el }</td> })
                }
            </tr>
        }
    }
}