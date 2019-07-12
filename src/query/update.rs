use crate::query::*;

impl<A: Column> Update<A> {
    fn make_set(&self, clause: &Vec<SetClause>) -> Result<String, ()> {
        match clause.as_slice() {
            [] => Err(()),
            _ => {
                let a = clause.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");

                Ok(a)
            }
        }
    }
}

impl<A: Column> ToSql for Update<A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("UPDATE");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_from(&state.from_clause) {
            sql = sql + " " + &a;
        }

        if let Ok(a) = self.make_set(&state.set_clause) {
            sql = sql + " SET " + &a;
        }

        if let Ok(a) = self.make_where(&state.where_clause) {
            sql = sql + " WHERE " + &a;
        }

        sql
    }
}

impl<A, B> UpdateSelect<A, B>
where
    A: HasEntityDef,
    B: HasSelect,
{
    fn make_table(&self) -> Result<String, ()> {
        Ok(A::table_name().name())
    }

    fn make_set(&self, clause: &Vec<SetClause>) -> Result<String, ()> {
        match clause.as_slice() {
            [] => Err(()),
            _ => {
                let a = clause.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
                Ok(a)
            }
        }
    }
}

impl<A: HasEntityDef, B: HasSelect> ToSql for UpdateSelect<A, B> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("UPDATE ");
        let state = self.0.state.borrow();
        let select_state = self.1.get_state();

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Ok(a) = self.make_set(&state.set_clause) {
            sql = sql + " SET " + &a;
        }

        if let Ok(a) = self.make_from(&select_state.from_clause) {
            sql = sql + " FROM " + &a;
        }

        if let Ok(a) = self.make_where(&select_state.where_clause) {
            sql = sql + " WHERE " + &a;
        }

        if let Ok(a) = self.make_limit(&select_state.limit_clause) {
            sql = sql + " " + &a;
        }

        sql
    }
}
