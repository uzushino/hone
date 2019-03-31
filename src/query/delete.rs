use crate::query::*;

impl<A> Delete<A> {}

impl<A: Column> ToSql for Delete<A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("DELETE");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_from(&state.from_clause) {
            sql = sql + " FROM " + &a;
        }
        if let Ok(a) = self.make_where(&state.where_clause) {
            sql = sql + " WHERE " + &a;
        }
        if let Ok(a) = self.make_group(&state.groupby_clause) {
            sql = sql + " GROUP BY " + &a;
        }
        if let Ok(a) = self.make_limit(&state.limit_clause) {
            sql = sql + " " + &a;
        }

        sql
    }
}

impl<A> Truncate <A> {}

impl<A: Column> ToSql for Truncate <A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("TRUNCATE TABLE ");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_from(&state.from_clause) {
            sql = sql + &a;
        }

        sql
    }
}
