#[macro_export]
macro_rules! column {
($a:expr) => {{
    $crate::table::Column {
        data_property: Some($a.to_string()),
        name: $a.to_string(),
        short_name: Some($a.to_string()),
    }
}};
($a:expr, $b:expr) => {{
    $crate::table::Column {
        data_property: Some($a.to_string()),
        name: $b.to_string(),
        short_name: Some($b.to_string()),
    }
}};
($a:expr, $b:expr, $c:expr) => {
    $crate::table::Column {
        data_property: Some($a.to_string()),
        name: $b.to_string(),
        short_name: Some($c.to_string()),
    }
};
}

#[macro_export]
macro_rules! columns {
( $( ( $($args:expr),* ) )+ ) => {
    vec![$(
        $crate::column![$($args),*]
    ),+];
};
}