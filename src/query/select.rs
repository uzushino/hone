use crate::query::*;

impl<'a, A: Column> Select<'a, A> {
    fn make_select(&self, distinct: &DistinctClause) -> Result<String, ()> {
        let kind = distinct.to_string();
        Ok(kind + &self.0.value.cols())
    }
}

impl<'a, A: Column> ToSql for Select<'a, A> {
    fn to_sql(&self) -> String {
        let mut sql = String::default();
        let state = self.get_state();

        if let Ok(a) = self.make_select(&state.distinct_clause) {
            sql = sql + "SELECT " + &a;
        }
        if let Ok(a) = self.make_from(&state.from_clause) {
            sql = sql + " FROM " + &a;
        }
        if let Ok(a) = self.make_where(&state.where_clause) {
            sql = sql + " WHERE " + &a;
        }
        if let Ok(a) = self.make_order(&state.order_clause) {
            sql = sql + " ORDER BY " + &a;
        }
        if let Ok(a) = self.make_group(&state.groupby_clause) {
            sql = sql + " GROUP BY " + &a;
        }
        if let Ok(a) = self.make_having(&state.having_clause) {
            sql = sql + " HAVING " + &a;
        }
        if let Ok(a) = self.make_limit(&state.limit_clause) {
            sql = sql + " " + &a;
        }

        sql
    }
}
