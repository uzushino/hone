use crate::query::*;

use self::column::*;
use self::from::*;

impl<A> Delete<A> {
    fn make_where(&self) -> Result<String, ()> {
        Ok(self.0.state.borrow().where_clause.to_string())
    }

    fn make_from(&self) -> Result<String, ()> {
        let fc = combine_joins(self.0.state.borrow().from_clause.as_slice(), &mut [])?;

        let from_str = fc.into_iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",");

        Ok(from_str)
    }

    pub fn make_limit(&self) -> Result<String, ()> {
        match self.0.state.borrow().limit_clause {
            LimitClause::Limit(_, _) => Ok(self.0.state.borrow().limit_clause.to_string()),
            LimitClause::No => Err(()),
        }
    }
}

impl<A: Column> HasDelete for Delete<A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("DELETE");

        if let Ok(a) = self.make_from() {
            sql = sql + " FROM " + &a;
        }

        if let Ok(a) = self.make_where() {
            sql = sql + " WHERE " + &a;
        }

        if let Ok(a) = self.make_limit() {
            sql = sql + " " + &a;
        }

        sql
    }
}
