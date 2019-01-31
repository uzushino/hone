use std::rc::Rc;

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
    pub state: QueryState,
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
}

pub fn select<A: Column>(q: Query<A>) -> impl HasSelect {
    Select(q)
}

pub struct Delete<A>(Query<A>);

pub trait HasDelete {
    fn to_sql(&self) -> String;
}

pub fn delete<A: Column>(q: Query<A>) -> impl HasDelete {
    Delete(q)
}

pub struct Update<A>(Query<A>);

pub trait HasUpdate {
    fn to_sql(&self) -> String;
}

pub fn update<A: Column>(q: Query<A>) -> impl HasUpdate {
    Update(q)
}

pub struct InsertInto<A>(Query<A>);
pub struct InsertSelect<A>(Select<A>);

pub trait HasInsert {
    fn to_sql(&self) -> String;
}

pub fn insert_into<A: HasEntityDef>(q: Query<A>) -> impl HasInsert {
    InsertInto(q)
}

pub fn insert_select<A: HasEntityDef>(q: Select<A>) -> impl HasInsert {
    InsertSelect(q)
}

pub trait UnsafeSqlFunctionArgument {
    fn to_arg_list(arg: Self) -> Vec<Rc<HasValue<(), bool>>>;
}
