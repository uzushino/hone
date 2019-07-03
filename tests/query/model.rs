use std::rc::Rc;

use hone::entity::*;
use hone::query::*;
use hone::types::*;

#[derive(Debug, Default, Clone)]
pub struct User {}

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

hone_model!(User, User, user_id => u32, email => String);
hone_entity!(User, User, email, user_id);

#[derive(Debug, Default, Clone)]
pub struct Library();

hone_model!(Library, Library, library_id => u32, title => String);
hone_entity!(Library, Library, title, library_id);