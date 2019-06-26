use std::rc::Rc;

use hone::entity::*;
use hone::query::*;
use hone::types::*;

#[derive(Debug, Default, Clone)]
pub struct User {}

impl User {
    pub fn user_id(&self) -> Box<HasValue<u32, Output=Column>> {
        Box::new(Column::new("User.user_id"))
    }

    pub fn email(&self) -> Box<HasValue<String, Output=Column>> {
        Box::new(Column::new("User.email"))
    }
}

impl HasEntityDef for User {
    fn table_name() -> Table {
        Table::new("User", None)
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "email",
            "user_id",
        ]
    }
}

impl HasQuery for User {
    type T = User;
}

#[derive(Debug, Default, Clone)]
pub struct Library();

impl Library {
    pub fn library_id(&self) -> Box<HasValue<u32, Output=Column>> {
        Box::new(Column::new("Library.library_id"))
    }

    pub fn title(&self) -> Box<HasValue<String, Output=Column>> {
        Box::new(Column::new("Library.title"))
    }
}

impl HasEntityDef for Library {
    fn table_name() -> Table {
        Table::new("Library", None)
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "library_id",
            "title"
        ]
    }
}

impl HasQuery for Library {
    type T = Library;
}
