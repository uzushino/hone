use crate::query::*;

use self::column::*;
use self::from::*;

impl<A: Column> Update<A> {
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
        let mut sql = String::from("UPDATE");

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

impl<A, B> UpdateSelect<A, B> where A: HasEntityDef, B: HasSelect {
    fn make_table(&self) -> Result<String, ()> {
        let ed = A::entity_def();
        Ok(ed.table_name.name())
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
    
    fn make_from(&self) -> Result<String, ()> {
        let st = self.1.get_state();
        let fc = combine_joins(st.from_clause.as_slice(), &mut [])?;

        let from_str = fc
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",");

        Ok(from_str)
    }
    
    fn make_where(&self) -> Result<String, ()> {
        let st = self.1.get_state();

        match st.where_clause {
            WhereClause::No => Err(()),
            _ => Ok(st.where_clause.to_string())
        }
    }

    pub fn make_limit(&self) -> Result<String, ()> {
        match self.0.state.limit_clause {
            LimitClause::Limit(_, _) => Ok(self.0.state.limit_clause.to_string()),
            LimitClause::No => Err(())
        }
    }
}


impl<A: HasEntityDef, B: HasSelect> HasUpdate for UpdateSelect<A, B> {
    fn to_sql(&self) -> String {
        let mut sql = String::from("UPDATE ");

        if let Ok(a) = self.make_table() {
            sql = sql + &a;
        }
        
        if let Ok(a) = self.make_set() {
            sql = sql + " SET " + &a;
        }

        if let Ok(a) = self.make_from() {
            sql = sql + " FROM " + &a;
        }
        
        if let Ok(a) = self.make_where() {
            sql = sql + " WHERE " + &a;
        }
        
        if let Ok(a) = self.make_limit() {
            sql = sql + " " + &a;
        }

        sql
    }
}