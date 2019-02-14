use crate::query::*;
use crate::types::QueryState;

use self::from::combine_joins;

impl<A: Column> Select<A> {
    pub fn make_select(&self) -> Result<String, ()> {
        Ok(self.0.value.cols())
    }

    fn make_where(&self) -> Result<String, ()> {
        match self.0.state.borrow().where_clause {
            WhereClause::No => Err(()),
            _ => Ok(self.0.state.borrow().where_clause.to_string()),
        }
    }

    fn make_order(&self) -> Result<String, ()> {
        if self.0.state.borrow().order_clause.is_empty() {
            return Err(());
        }

        let a = self
            .0
            .state
            .borrow()
            .order_clause
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Ok(a)
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

    pub fn make_group(&self) -> Result<String, ()> {
        if self.0.state.borrow().groupby_clause.is_empty() {
            return Err(());
        };

        let c = self
            .0
            .state
            .borrow()
            .groupby_clause
            .iter()
            .map(|v| (*v).to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Ok(c)
    }
}

impl<A: Column> HasSelect for Select<A> {
    fn to_sql(&self) -> String {
        let mut sql = String::default();

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
        if let Ok(a) = self.make_group() {
            sql = sql + " GROUP BY " + &a;
        }
        if let Ok(a) = self.make_limit() {
            sql = sql + " " + &a;
        }

        sql
    }

    fn get_state(&self) -> Ref<QueryState> {
        self.0.state.borrow()
    }
}
