use crate::query::*;

use self::column::*;
use self::from::*;

impl<A> Update<A> {
    fn make_where(&self) -> Result<String, ()> {
        Ok(self.0.state.where_clause.to_string())
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
    
    fn make_set(&self) -> Result<String, ()> {
        if self.0.state.set_clause.is_empty() {
            return Err(());
        }

        let a = self.0.state.set_clause
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        Ok(a)
    }
}

impl<A: Column> HasUpdate for Update<A> {
    fn to_sql(&self) -> String {
        let mut sql: String = "UPDATE".into();

        if let Ok(a) = self.make_from() {
            sql = sql + " " + &a;
        }
        
        if let Ok(a) = self.make_set() {
            sql = sql + " SET " + &a;
        }

        if let Ok(a) = self.make_where() {
            sql = sql + " WHERE " + &a;
        }

        sql
    }
}