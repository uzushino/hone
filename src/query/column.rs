use std::rc::Rc;
use crate::entity::HasEntityDef;
use crate::types::*;

pub trait Column {
    fn cols(&self) -> String;
    fn col_count() -> usize;
}

impl<A: ToString, DB: ToLiteral> Column for Box<HasValue<A, Output=DB>> {
    fn cols(&self) -> String {
        self.to_string()
    }

    fn col_count() -> usize {
        1
    }
}

impl<A> Column for A where A: HasEntityDef {
    fn cols(&self) -> String {
        A::columns().join(", ")
    }

    fn col_count() -> usize {
        A::columns().len()
    }
}

impl<A, B> Column for (A, B) where A: Column, B: Column {
    fn cols(&self) -> String {
        let ca = self.0.cols();
        let cb = self.1.cols();

        ca + ", " + &cb
    }

    fn col_count() -> usize {
        A::col_count() + B::col_count()
    }
}

impl<A, B, C> Column for (A, B, C) where A: Column, B: Column, C: Column {
    fn cols(&self) -> String {
        let ca = self.0.cols();
        let cb = self.1.cols();
        let cc = self.2.cols();

        ca + ", " + &cb + ", " + &cc
    }

    fn col_count() -> usize {
        A::col_count() + B::col_count() + C::col_count()
    }
}
