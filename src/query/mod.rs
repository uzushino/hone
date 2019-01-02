use std::borrow::Borrow;
use std::rc::Rc;

use types::*;

mod from;
mod select;

use self::select::SqlSelect;

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

impl<A: SqlSelect> Query<A> {
    pub fn to_sql(&self) -> String {
        let mut sql: String = "".into();

        if let Ok(a) = self.make_select() {
            sql = sql + "SELECT " + &a;
        }
        if let Ok(a) = self.make_from() {
            sql = sql + " FROM " + &a;
        }
        if let Ok(a) = self.make_where() {
            sql = sql + " WHERE " + &a;
        }
        if let Ok(a) = self.make_order() {
            sql = sql + " ORDER BY " + &a;
        }

        sql
    }

    fn make_select(&self) -> Result<String, ()> {
        Ok(self.value.cols())
    }

    fn make_where(&self) -> Result<String, ()> {
        Ok(self.state.where_clause.clone().into())
    }

    fn make_order(&self) -> Result<String, ()> {
        if self.state.order_clause.is_empty() {
            return Err(());
        }

        let a = self.state.order_clause.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");

        Ok(a)
    }

    fn make_from(&self) -> Result<String, ()> {
        let fc = combine_joins(self.state.from_clause.as_slice(), &mut [])?;

        let from_str = fc.into_iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",");

        Ok(from_str)
    }
}

fn set_on(join: &FromClause, on: &Rc<HasValue<bool>>) -> Option<FromClause> {
    match join.clone() {
        FromClause::Join(lhs, knd, rhs, on_) => {
            if let Some(f) = set_on(rhs.borrow(), on) {
                return Some(FromClause::Join(lhs, knd, Rc::new(f), on_));
            }

            if let Some(f) = set_on(lhs.borrow(), on) {
                return Some(FromClause::Join(Rc::new(f), knd, rhs, on_));
            }

            match on_ {
                None => Some(FromClause::Join(lhs, knd, rhs, Some(on.clone()))),
                _ => None,
            }
        }
        _ => None,
    }
}

fn find_imcomplete_and_set_on(joins: &[FromClause], on: Rc<HasValue<bool>>) -> either::Either<Rc<HasValue<bool>>, Vec<FromClause>> {
    match joins.split_first() {
        Some((ref join, rest)) => {
            if let Some(f) = set_on(*join, &on) {
                let mut rest = rest.to_vec();
                rest.push(f.clone());

                return either::Right(rest);
            }

            let mut v = try_right!(find_imcomplete_and_set_on(rest, on));

            v.insert(0, (*join).clone());
            either::Right(v)
        }
        None => either::Left(on),
    }
}

fn combine_joins(fs: &[FromClause], acc: &mut [FromClause]) -> Result<Vec<FromClause>, ()> {
    match fs.split_first() {
        Some((&FromClause::OnClause(ref on), rest)) => match find_imcomplete_and_set_on(acc, on.clone()) {
            either::Right(mut acc_) => combine_joins(rest, acc_.as_mut_slice()),
            either::Left(_) => Err(()),
        },
        Some((ref head, rest)) => {
            let mut acc = acc.to_vec();
            acc.push((*head).clone());

            combine_joins(rest, acc.as_mut_slice())
        }
        _ => {
            acc.reverse();
            Ok(acc.to_vec())
        }
    }
}
