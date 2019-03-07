use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::entity::HasEntityDef;
use crate::types::*;

mod column;
mod delete;
mod from;
mod functions;
mod insert;
mod select;
mod update;

use self::column::*;
use self::from::combine_joins;

pub trait HasQuery {
    type T;
}

pub struct Query<A> {
    pub state: Rc<RefCell<QueryState>>,
    pub value: A,
}

pub trait ToSql {
    fn to_sql(&self) -> String;

    fn make_where(&self, clause: &WhereClause) -> Result<String, ()> {
        match clause {
            WhereClause::No => Err(()),
            _ => Ok(clause.to_string()),
        }
    }

    fn make_order(&self, clause: &Vec<OrderClause>) -> Result<String, ()> {
        match clause.as_slice() {
            [] => Err(()),
            _ => {
                let s = clause.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
                Ok(s)
            }
        }
    }

    fn make_from(&self, clause: &Vec<FromClause>) -> Result<String, ()> {
        let fc = combine_joins(clause.as_slice(), &mut [])?;
        let s = fc.into_iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",");

        Ok(s)
    }

    fn make_limit(&self, clause: &LimitClause) -> Result<String, ()> {
        match clause {
            LimitClause::Limit(_, _) => Ok(clause.to_string()),
            LimitClause::No => Err(()),
        }
    }

    fn make_group(&self, clause: &Vec<GroupByClause>) -> Result<String, ()> {
        if clause.is_empty() {
            return Err(());
        };

        let c = clause.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");

        Ok(c)
    }

    fn make_having(&self, clause: &WhereClause) -> Result<String, ()> {
        match clause {
            WhereClause::No => Err(()),
            _ => Ok(clause.to_string()),
        }
    }
}

pub trait FromQuery {
    type Kind;

    fn from_() -> Result<Query<Self::Kind>, ()>;
    fn from_by<F, R>(f: F) -> Result<Query<R>, ()>
    where
        F: Fn(Query<Self::Kind>, Self::Kind) -> Query<R>;
}

pub trait HasSelect: ToSql {
    fn get_state(&self) -> Ref<QueryState>;
}
impl<A: Column> HasSelect for Select<A> {
    fn get_state(&self) -> Ref<QueryState> {
        self.0.state.borrow()
    }
}

pub struct Select<A>(Query<A>);

pub fn select<A: Column>(q: Query<A>) -> impl HasSelect {
    Select(q)
}

pub trait HasUpdate: ToSql {}

pub struct Update<A>(Query<A>);
impl<A: Column> HasUpdate for Update<A> {}

pub struct UpdateSelect<A, B: HasSelect>(Query<A>, B);
impl<A: HasEntityDef, B: HasSelect> HasUpdate for UpdateSelect<A, B> {}

pub fn update<A: Column>(q: Query<A>) -> impl HasUpdate {
    Update(q)
}

pub fn update_select<A: Column, B, F>(q: Query<A>, f: F) -> UpdateSelect<B, impl HasSelect>
where
    F: Fn(Query<B>, B, &Query<A>) -> Query<B>,
    B: Default,
{
    let qs = Query::new(B::default());
    let qs = f(qs, B::default(), &q);

    UpdateSelect(qs, Select(q))
}

pub trait HasInsert: ToSql {}

pub struct InsertInto<A>(Query<A>);
impl<A: HasEntityDef> HasInsert for InsertInto<A> {}

pub struct InsertSelect<A, B: HasSelect>(Query<A>, B);
impl<A: HasEntityDef, B: HasSelect> HasInsert for InsertSelect<A, B> {}

pub fn insert_into<A: HasEntityDef>(q: Query<A>) -> impl HasInsert {
    InsertInto(q)
}

pub fn insert_select<A: Column, B, F>(q: Query<A>, f: F) -> InsertSelect<B, impl HasSelect>
where
    F: Fn(Query<B>, B, &Query<A>) -> Query<B>,
    B: Default,
{
    let qs = Query::new(B::default());
    let qs = f(qs, B::default(), &q);

    InsertSelect(qs, Select(q))
}

pub trait HasDelete: ToSql {}

pub struct Delete<A>(Query<A>);
impl<A: Column> HasDelete for Delete<A> {}

pub fn delete<A: Column>(q: Query<A>) -> impl HasDelete {
    Delete(q)
}

pub trait UnsafeSqlFunctionArgument {
    fn to_arg_list(arg: Self) -> Vec<Rc<HasValue<(), bool>>>;
}
