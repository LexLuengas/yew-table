//! yew-table macros


/// Macro to generate a column struct provided variable number of arguments
#[macro_export]
macro_rules! column {
    ($a:expr) => {{
        $crate::types::Column {
            data_property: Some($a.to_string()),
            name: $a.to_string(),
            short_name: Some($a.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    }};
    ($a:expr, $b:expr) => {{
        $crate::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($b.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    }};
    ($a:expr, $b:expr, $c:expr) => {
        $crate::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($c.to_string()),
            orderable: false,
            header_classes: vec![],
        }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::types::Column {
            data_property: Some($a.to_string()),
            name: $b.to_string(),
            short_name: Some($c.to_string()),
            orderable: $d,
            header_classes: vec![],
        }
    };
}


/// Macro to generate a vector of columns providing variable number of arguments
#[macro_export]
macro_rules! columns {
( $( ( $($args:expr),* ) )+ ) => {
    vec![$(
        $crate::column![$($args),*]
    ),+]
};
}
