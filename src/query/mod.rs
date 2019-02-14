use std::rc::Rc;
use std::cell::{RefCell, Ref};

use crate::types::*;
use crate::entity::HasEntityDef;

mod column;
mod from;
mod functions;
mod select;
mod delete;
mod update;
mod insert;

use self::column::*;

pub trait HasQuery {
    type T;
}

pub struct Query<A> {
    pub state: Rc<RefCell<QueryState>>,
    pub value: A,
}

pub trait FromQuery {
    type Kind;

    fn from_() -> Result<Query<Self::Kind>, ()>;
    fn from_by<F, R>(f: F) -> Result<Query<R>, ()>
    where
        F: Fn(Query<Self::Kind>, Self::Kind) -> Query<R>;
}

pub struct Select<A>(Query<A>);

pub trait HasSelect {
    fn to_sql(&self) -> String;

    fn get_state(&self) -> Ref<QueryState>;
}

pub fn select<A: Column>(q: Query<A>) -> impl HasSelect {
    Select(q)
}

pub struct Update<A>(Query<A>);
pub struct UpdateSelect<A, B: HasSelect>(Query<A>, B);

pub trait HasUpdate {
    fn to_sql(&self) -> String;
}

pub fn update<A: Column>(q: Query<A>) -> impl HasUpdate { 
    Update(q)
}

pub fn update_select<A: Column, B, F>(q: Query<A>, f: F) -> UpdateSelect<B, impl HasSelect>
where F: Fn(Query<B>, B, &Query<A>) -> Query<B>, B: Default {
    let qs = Query::new(B::default());
    let qs = f(qs, B::default(), &q);

    UpdateSelect(qs, Select(q))
}

pub struct InsertInto<A>(Query<A>);
pub struct InsertSelect<A, B: HasSelect>(Query<A>, B);

pub trait HasInsert {
    fn to_sql(&self) -> String;
}

pub fn insert_into<A: HasEntityDef>(q: Query<A>) -> impl HasInsert {
    InsertInto(q)
}

pub fn insert_select<A: Column, B, F>(q: Query<A>, f: F) -> InsertSelect<B, impl HasSelect>
where F: Fn(Query<B>, B, &Query<A>) -> Query<B>, B: Default {
    let qs = Query::new(B::default());
    let qs = f(qs, B::default(), &q);

    InsertSelect(qs, Select(q))
}

pub struct Delete<A>(Query<A>);

pub trait HasDelete {
    fn to_sql(&self) -> String;
}

pub fn delete<A: Column>(q: Query<A>) -> impl HasDelete {
    Delete(q)
}

pub trait UnsafeSqlFunctionArgument {
    fn to_arg_list(arg: Self) -> Vec<Rc<HasValue<(), bool>>>;
}
