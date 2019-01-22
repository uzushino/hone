use std::rc::Rc;

use crate::types::*;

mod column;
mod from;
mod select;
mod functions;
mod delete;

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

pub trait UnsafeSqlFunctionArgument {
    fn to_arg_list(arg: Self) -> Vec<Rc<HasValue<(), bool>>>;
}
