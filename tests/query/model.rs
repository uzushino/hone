use std::rc::Rc;

use hone::entity::*;
use hone::query::*;
use hone::types::*;
use hone::{hone_model, hone_entity};

#[derive(Debug, Default, Clone)]
pub struct User {}

hone_model!(User, User, user_id => u32, email => String);
hone_entity!(User, User, email, user_id);

#[derive(Debug, Default, Clone)]
pub struct Library();

hone_model!(Library, Library, library_id => u32, title => String);
hone_entity!(Library, Library, library_id, title);