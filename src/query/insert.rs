use crate::entity::HasEntityDef;
use crate::query::*;

impl<'a, A> InsertInto<'a, A> where A: HasEntityDef {
    fn make_table(&self) -> Result<String, ()> {
        Ok(A::table_name().name())
    }

    fn make_column(&self, clause: &Vec<SetClause>) -> Result<String, ()> {
        let s = clause.iter().map(|f| f.column()).collect::<Vec<_>>().join(", ");
        Ok(s)
    }

    fn make_values(&self, clause: &Vec<SetClause>) -> Result<String, ()> {
        let s = clause.iter().map(|f| f.value()).collect::<Vec<_>>().join(", ");
        Ok(s)
    }
}

impl<'a, A: HasEntityDef> ToSql for InsertInto<'a, A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("INSERT INTO ");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Ok(a) = self.make_column(&state.set_clause) {
            sql = sql + "(" + &a + ")";
        }

        if let Ok(a) = self.make_values(&state.set_clause) {
            sql = sql + " VALUES " + "(" + &a + ")";
        }

        sql
    }
}

impl<'a, A, B> InsertSelect<'a, A, B> where A: HasEntityDef, B: HasSelect {
    fn make_table(&self) -> Result<String, ()> {
        Ok(A::table_name().name())
    }

    fn make_column(&self, clause: &Vec<SetClause>) -> Result<String, ()> {
        let s = clause.iter().map(|f| f.column()).collect::<Vec<_>>().join(", ");
        Ok(s)
    }
}

impl<'a, A: HasEntityDef, B: HasSelect> ToSql for InsertSelect<'a, A, B> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("INSERT INTO ");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Ok(a) = self.make_column(&state.set_clause) {
            sql = sql + "(" + &a + ")";
        }

        sql + " " + self.1.to_sql().as_ref()
    }
}

impl<'a, A> BulkInsert<'a, A> where A: HasEntityDef {
    fn make_table(&self) -> Result<String, ()> {
        Ok(A::table_name().name())
    }
    
    fn make_column(&self, values: &Box<HasValues>) -> Result<String, ()> {
        let c = values.columns();

        if c.is_empty() {
            return Err(());
        }

        Ok(c.join(", "))
    }

    fn make_values(&self, clause: &Box<HasValues>) -> Result<String, ()> {
        let values = clause.values()
            .iter()
            .map(|v| "(".to_string() + &v.join(", ") + ")")
            .collect::<Vec<String>>();
        
        Ok(values.join(" "))
    }
}

impl<'a, A: HasEntityDef> ToSql for BulkInsert<'a, A> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("INSERT INTO ");
        let state = self.0.state.borrow();

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Some(clause) = &state.values_clause {
            if let Ok(a) = self.make_column(clause) {
                sql = sql + "(" + &a + ")";
            }
        
            if let Ok(a) = self.make_values(&clause) {
                sql = sql + " VALUES " + &a ;
            }
        }

        sql
    }
}