use crate::query::*;

use self::column::*;
use self::from::*;

impl<A> Delete<A> {
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
}

impl<A: Column> HasDelete for Delete<A> {
    fn to_sql(&self) -> String {
        let mut sql: String = "DELETE".into();

        if let Ok(a) = self.make_from() {
            sql = sql + " FROM " + &a;
        }

        if let Ok(a) = self.make_where() {
            sql = sql + " WHERE " + &a;
        }

        sql
    }
}