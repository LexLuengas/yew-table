# Yew Table

A simple table component for the Yew web framework.

## Usage

*Use the Table component by setting the `columns` and `data` properties:*

```rust
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        // Define the columns. The first string is the field name, the second is the label.
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
                // Here, self.tasks is a vector of structs
                <Table<Task>: columns=columns, data=&self.tasks,/>
            </>
        }
    }
}
```

*Implement the TableData trait for the struct to be used:*

```rust
#[derive(Default, Clone, PartialEq, Serialize)]
pub struct Task {
    pub id: String,
    // ...
}

impl TableData for Task {
    fn get_field_as_html(&self, field_name: &str) -> Html<Table<Self>> {
        match field_name {
            // Define how each field should be rendered. No restrictions.
            "id" => html! {
                { &self.id }
            },
            // ...
        }
    }
}
```

## Example

An example Yew app showing a plain table can be found in the _examples_ folder. Just run the contained `run.sh` script. 

## To-dos

- [x] Add sortability
- [ ] Add searchability
- [ ] Improve accessibility
- [ ] Add pagination

## License

[MIT](LICENSE) © 2019 Alexis Luengas
