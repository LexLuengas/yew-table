use yew::prelude::*;
use yew::html;
use std::cmp::Reverse;
use crate::table::*;

/// Properties of the Table component.
#[derive(Clone, PartialEq, Default)]
pub struct TableProps<T> where T: TableData {
    pub columns: Vec<Column>,
    pub data: Vec<T>,
    pub options: Option<TableOptions>,
}

#[derive(Debug)]
pub enum Msg {
    SortColumn(usize),
}

impl<T> Component for Table<T> where T: TableData {
    type Message = Msg;
    type Properties = TableProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let column_number = props.columns.len();
        Table {
            columns: props.columns,
            data: props.data,
            options: props.options,
            state: TableState {
                order: vec![TableOrder::default(); column_number],
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SortColumn(i) => {
                use TableOrder::*;

                for (j, x) in self.state.order.iter_mut().enumerate() {
                    if j != i {
                        *x = Unordered
                    } else {
                        *x = x.rotate()
                    }
                }
                
                match self.columns[i].data_property.as_ref() {
                    Some(f) => {
                        match self.state.order[i] {
                            Unordered => self.data.sort(),
                            Ascending => self.data.sort_by_cached_key(|x| x.get_field_as_value(&f).unwrap()),
                            Descending => self.data.sort_by_cached_key(|x| Reverse(x.get_field_as_value(&f).unwrap())),
                        }
                        true
                    },
                    None => false
                }
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.columns = props.columns;
        self.data = props.data;
        true
    }
}

impl<T> Renderable<Table<T>> for Table<T> where T: TableData {
    fn view(&self) -> Html<Self> {
        let get_orderable_class = || {
            if self.is_orderable() {
                "is-orderable"
            } else {
                ""
            }
        }; 

        html! {
            <table class=("yew-table", get_orderable_class()),>
                <thead>
                    { for self.columns.iter().enumerate().map(|(i, col)| self.view_column(i, &col)) }
                </thead>
                <tbody>
                    { for self.data.iter().map(|d| self.view_row(&d)) }
                </tbody>
            </table>
        }
    }
}

impl<T> Table<T> where T: TableData {
    fn view_column<'a>(&'a self, index: usize, column: &'a Column) -> Html<Table<T>> {
        let get_header_sorting_class = |index: usize| {
            use TableOrder::*;
            match self.state.order[index] {
                Unordered => "",
                Ascending => "is-sorting-ascending",
                Descending => "is-sorting-descending",
            }
        };

        let th_view = |child| {
            if self.is_orderable() {
                html! { <th onclick=|_| Msg::SortColumn(index),>{ child }</th> }
            } else {
                html! { <th>{ child }</th> }
            }
        };

        th_view(html! {
            <span>
                <abbr title=&column.name,>
                    { column }
                </abbr><i class=("sorting-control", get_header_sorting_class(index)),></i>
            </span>
        })
    }

    fn view_row(&self, datum: &T) -> Html<Table<T>> {
        html! {
            <tr>
                { 
                    for self.columns.iter()
                        .map(|c| { c.data_property.as_ref().unwrap_or(&c.name) })
                        .map(|name| { datum.get_field_as_html(name) })
                        .filter_map(|h| h.ok())
                        .map(|el| html! { <td>{ el }</td> })
                }
            </tr>
        }
    }
}