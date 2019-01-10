use std::collections::HashMap;
use std::marker::PhantomData;

use types::*;

#[derive(Clone)]
pub struct Entity<T>(PhantomData<T>);

#[derive(Clone, Default)]
pub struct Column(String, Option<String>);

impl Column {
    pub fn new(name: &str) -> Column {
        Column(name.to_string(), None)
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl ToString for Column {
    fn to_string(&self) -> String {
        self.name()
    }
}

impl<A: IntoSql> HasValue<A> for Column {
    fn to_sql(&self) -> String where Self: Sized {
        A::sql_value(self.0.clone())
    }
}

#[derive(Clone, Default)]
pub struct Table(String, Option<String>);

impl Table {
    pub fn new(name: &str, alias: Option<String>) -> Table {
        Table(name.to_string(), alias)
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Default)]
pub struct EntityDef {
    pub table_name: Table,
    pub columns: HashMap<String, String>,
}

pub trait HasEntityDef {
    fn entity_def() -> EntityDef;
}

impl<T> Default for Entity<T> {
    fn default() -> Entity<T> {
        Entity(PhantomData)
    }
}

impl<T: HasEntityDef> HasEntityDef for Entity<T> {
    fn entity_def() -> EntityDef {
        T::entity_def()
    }
}

impl<T: HasEntityDef> HasEntityDef for Option<T> {
    fn entity_def() -> EntityDef {
        T::entity_def()
    }
}
