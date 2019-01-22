use crate::query::*;

use self::from::combine_joins;

impl<A: Column> Select<A> {
    fn make_select(&self) -> Result<String, ()> {
        Ok(self.0.value.cols())
    }

    fn make_where(&self) -> Result<String, ()> {
        match self.0.state.where_clause {
            WhereClause::No => Err(()),
            _ => Ok(self.0.state.where_clause.to_string())
        }
    }

    fn make_order(&self) -> Result<String, ()> {
        if self.0.state.order_clause.is_empty() {
            return Err(());
        }

        let a = self.0.state.order_clause
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Ok(a)
    }

    fn make_from(&self) -> Result<String, ()> {
        let fc = combine_joins(self.0.state.from_clause.as_slice(), &mut [])?;

        let from_str = fc
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        Ok(from_str)
    }
}

impl<A: Column> HasSelect for Select<A> {
    fn to_sql(&self) -> String {
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
}