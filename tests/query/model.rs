use std::rc::Rc;

use hone::entity::*;
use hone::query::*;
use hone::types::*;
use hone::{hone_entity, hone_model};

#[derive(Debug, Default, Clone)]
pub struct User {}

impl User {
    pub fn user_id(&self) -> Rc<HasValue<u32, Output=Column>> {
        Rc::new(Column::new(format!("{}.{}", "User", "user_id").as_str()))
    }
    
    pub fn user_id_(&self) -> Rc<HasValue<u32, Output=Column>> {
        Rc::new(Column::new(format!("{}", "user_id").as_str()))
    }
    
    pub fn email(&self) -> Rc<HasValue<String, Output=Column>> {
        Rc::new(Column::new(format!("{}.{}", "User", "email").as_str()))
    }

    pub fn email_(&self) -> Rc<HasValue<String, Output=Column>> {
        Rc::new(Column::new(format!("{}", "email").as_str()))
    }
}

impl HasQuery for User {
    type T = User;
}

hone_entity!(User, User, email, user_id);

#[derive(Debug, Default, Clone)]
pub struct Library();

hone_model!(Library, Library, library_id => u32, title => String);
hone_entity!(Library, Library, library_id, title);
