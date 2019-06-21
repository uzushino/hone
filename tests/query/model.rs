use std::collections::HashMap;
use std::rc::Rc;

use hone::entity::*;
use hone::query::*;
use hone::types::*;

#[derive(Debug, Default, Clone)]
pub struct User {}

impl User {
    pub fn user_id(&self) -> Rc<HasValue<u32, Output=Column>> {
        Rc::new(Column::new("User.user_id"))
    }

    pub fn email(&self) -> Rc<HasValue<String, Output=Column>> {
        Rc::new(Column::new("User.email"))
    }
}

impl HasEntityDef for User {
    fn entity_def() -> EntityDef {
        let mut m = HashMap::new();

        m.insert("user_id".to_string(), "integer".to_string());
        m.insert("email".to_string(), "text".to_string());

        EntityDef {
            table_name: Table::new("User", None),
            columns: m,
        }
    }
}

impl HasQuery for User {
    type T = User;
}

#[derive(Debug, Default, Clone)]
pub struct Library();

impl Library {
    pub fn library_id(&self) -> Rc<HasValue<u32, Output=Column>> {
        Rc::new(Column::new("Library.library_id"))
    }

    pub fn title(&self) -> Rc<HasValue<String, Output=Column>> {
        Rc::new(Column::new("Library.title"))
    }
}

impl HasEntityDef for Library {
    fn entity_def() -> EntityDef {
        let mut m = HashMap::new();

        m.insert("library_id".to_string(), "integer".to_string());
        m.insert("title".to_string(), "text".to_string());

        EntityDef {
            table_name: Table::new("Library", None),
            columns: m,
        }
    }
}

impl HasQuery for Library {
    type T = Library;
}
