use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt;

use crate::types::*;

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

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<A, DB: ToLiteral> HasValue<A, DB> for Column {
    fn to_sql(&self) -> String where Self: Sized {
        DB::to_literal(self.0.clone())
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
