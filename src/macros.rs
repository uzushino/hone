#[macro_export]
macro_rules! hone_model {
    ($model:ident, $table:ident, $($column:ident => $type:ty),+ ) => {
        impl $model {
            $(
                pub fn $column(&self) -> Rc<HasValue<$type, Output=Column>> {
                    Rc::new(Column::new(format!("{}.{}", stringify!($table), stringify!($column)).as_str()))
                }
            )*
        }

        impl HasQuery for $model {
            type T = $model;
        }
    }
}

#[macro_export]
macro_rules! hone_entity {
    ($model:ident, $table:ident, $($col:tt),* ) => {
        impl HasEntityDef for $model {
            fn table_name() -> Table {
                Table::new(stringify!($table), None)
            }

            fn columns() -> Vec<&'static str> {
                let mut result = vec![];
                $(
                    result.push(stringify!($col));
                )*
                result
            }
        }
    };
}