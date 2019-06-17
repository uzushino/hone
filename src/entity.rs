use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use crate::types::*;

#[derive(Clone)]
pub struct Entity<T>(PhantomData<T>);

#[derive(Clone, Default)]
pub struct Column<A>(String, Option<String>, std::marker::PhantomData<A>);

impl<B: ToLiteral> Column<B> {
    pub fn new(name: &str) -> Column<B> {
        Column(name.to_string(), None, std::marker::PhantomData)
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl<B: ToLiteral> fmt::Display for Column<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<A, B: ToLiteral> HasValue<A> for Column<B> {
    type Output = B;

    fn to_sql(&self) -> String where Self: Sized {
        Self::Output::to_literal(self.0.clone())
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
