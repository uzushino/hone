use std::collections::BTreeMap;
use std::rc::Rc;

use crate::entity::*;
use crate::types::*;

pub trait SqlSelect {
    fn cols(&self) -> String;
    fn col_count() -> usize;
}

impl<A: ToString, DB> SqlSelect for Rc<HasValue<A, DB>> {
    fn cols(&self) -> String {
        self.to_string()
    }

    fn col_count() -> usize {
        1
    }
}

impl<A> SqlSelect for A where A: HasEntityDef {
    fn cols(&self) -> String {
        let ed = A::entity_def();
        let ordered: BTreeMap<_, _> = ed.columns.iter().collect();

        let s = ordered.keys().map(|k| k.clone().to_string()).collect::<Vec<_>>();

        s.join(", ")
    }

    fn col_count() -> usize {
        A::entity_def().columns.keys().count()
    }
}

impl<A, B> SqlSelect for (A, B) where A: SqlSelect, B: SqlSelect {
    fn cols(&self) -> String {
        let ca = self.0.cols();
        let cb = self.1.cols();

        ca + ", " + &cb
    }

    fn col_count() -> usize {
        A::col_count() + B::col_count()
    }
}
