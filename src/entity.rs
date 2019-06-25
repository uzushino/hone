use std::fmt;
use std::rc::Rc;
use std::marker::PhantomData;

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

impl<A: ToLiteral> HasValue<A> for Column {
    type Output = Column;

    fn to_sql(&self) -> String {
        Self::Output::to_literal(&self.0)
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

pub trait HasEntityDef {
    fn star() -> Rc<HasValue<String, Output=Column>> {
        let t = Self::table_name();
        Rc::new(Column::new(format!("{}.{}", t.name(), "*").as_str()))
    }

    fn table_name() -> Table;
    fn columns() -> Vec<&'static str>;
}
