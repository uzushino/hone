use crate::query::*;
use crate::types::QueryState;

impl<A: Column> Select<A> {
    fn get_state(&self) -> Ref<QueryState> {
        self.0.state.borrow()
    }

    fn make_select(&self) -> Result<String, ()> {
        Ok(self.0.value.cols())
    }
}
impl<A: Column> ToSql for Select<A> {
    fn to_sql(&self) -> String {
        let mut sql = String::default();
        let state = self.get_state();

        if let Ok(a) = self.make_select() {
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
        if let Ok(a) = self.make_limit(&state.limit_clause) {
            sql = sql + " " + &a;
        }

        sql
    }
}
