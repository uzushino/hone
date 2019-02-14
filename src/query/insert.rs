use crate::query::*;

use crate::entity::HasEntityDef;

impl<A> InsertInto<A>
where
    A: HasEntityDef,
{
    fn make_table(&self) -> Result<String, ()> {
        let ed = A::entity_def();
        Ok(ed.table_name.name())
    }

    fn make_column(&self) -> Result<String, ()> {
        let s = self.0.state.borrow().set_clause.iter().map(|f| f.column()).collect::<Vec<_>>().join(", ");

        Ok(s)
    }

    fn make_values(&self) -> Result<String, ()> {
        let s = self.0.state.borrow().set_clause.iter().map(|f| f.value()).collect::<Vec<_>>().join(", ");

        Ok(s)
    }
}

impl<A: HasEntityDef> HasInsert for InsertInto<A> {
    fn to_sql(&self) -> String {
        let mut sql: String = "INSERT INTO ".into();

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Ok(a) = self.make_column() {
            sql = sql + "(" + &a + ")";
        }

        if let Ok(a) = self.make_values() {
            sql = sql + " VALUES " + "(" + &a + ")";
        }

        sql
    }
}

impl<A, B> InsertSelect<A, B>
where
    A: HasEntityDef,
    B: HasSelect,
{
    fn make_table(&self) -> Result<String, ()> {
        let ed = A::entity_def();
        Ok(ed.table_name.name())
    }

    fn make_column(&self) -> Result<String, ()> {
        let s = self.0.state.borrow().set_clause.iter().map(|f| f.column()).collect::<Vec<_>>().join(", ");

        Ok(s)
    }
}

impl<A: HasEntityDef, B: HasSelect> HasInsert for InsertSelect<A, B> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("INSERT INTO ");

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }

        if let Ok(a) = self.make_column() {
            sql = sql + "(" + &a + ")";
        }

        sql = sql + " " + self.1.to_sql().as_ref();

        sql
    }
}
