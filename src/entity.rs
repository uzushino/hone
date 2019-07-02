use std::fmt;
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
        match &self.1 {
            &Some(ref n) => n.clone(),
            None => self.0.clone(),
        }
    }

    pub fn as_(&mut self, name: &str) -> Column {
        Column(self.0.to_string(), Some(name.to_string()))
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.1 {
            Some(ref s) => write!(f, "{}", s),
            _ => write!(f, "{}", self.0),
        }
        
    }
}

impl<A: ToString + ToLiteral> HasValue<A> for Column {
    type Output = Column;

    fn to_sql(&self) -> String {
        Self::Output::to_literal(&self.0)
    }
}

#[derive(Clone)]
pub struct Star;

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

impl<A: ToString + ToLiteral> HasValue<A> for Star {
    type Output = Column;

    fn to_sql(&self) -> String {
        "*".to_string()
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
    
    pub fn as_(&mut self, name: &str) -> Table {
        Table(self.0.to_string(), Some(name.to_string()))
    }
}

pub trait HasEntityDef {
    fn table_name() -> Table;
    fn columns() -> Vec<&'static str>;
}
