use std::default::Default;
use std::ops::Add;
use std::rc::Rc;

use crate::entity::{Entity, Column};
use crate::expression::and_;

#[derive(Debug, Clone)]
pub enum OrderByType {
    Asc,
    Desc,
}

// Expr (Entity val)
pub trait HasEntity {}

impl<A> HasEntity for Entity<A> {}

// Expr (Maybe a)
pub trait HasOption {}

impl<A> HasOption for Option<A> {}

// Expr (PreprocessedFrom a)
pub trait HasPreprocess {}

pub struct FromPreprocess<A>(pub A, pub FromClause);
impl<A> HasPreprocess for FromPreprocess<A> {}

pub trait ToLiteral {
    fn to_literal<A: ToString>(v: A) -> String ;
}

impl ToLiteral for String {
    fn to_literal<A: ToString>(v: A) -> String {
        format!("'{}'", v.to_string()) 
    }
}

impl ToLiteral for bool {
    fn to_literal<A: ToString>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for i32 {
    fn to_literal<A: ToString>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for u32 {
    fn to_literal<A: ToString>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for Column {
    fn to_literal<A: ToString>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

// Expr (Value a)
pub trait HasValue<A, DB> : ToString {
    fn to_sql(&self) -> String;
}

impl<A: std::fmt::Display, DB> HasValue<A, DB> for Rc<HasValue<A, DB>> where Self: ToString {
    fn to_sql(&self) -> String { self.to_string() }
}

#[derive(Clone)]
pub enum NeedParens {
    Never,
    Parens,
}

#[derive(Clone)]
pub struct Raw(pub NeedParens, pub String);

impl<A, DB: ToLiteral> HasValue<A, DB> for Raw {
    fn to_sql(&self) -> String where Self: Sized {
        let s = DB::to_literal(self.1.clone());

        match self.0 {
            NeedParens::Never => s.to_string(),
            NeedParens::Parens => "(".to_owned() + s.as_str() + ")",
        }
    }
}

impl ToString for Raw {
    fn to_string(&self) -> String {
        match self.0 {
            NeedParens::Never => self.1.to_string(),
            NeedParens::Parens => "(".to_owned() + &self.1 + ")",
        }
    }
}

pub struct CompositKey<A: ToString>(pub A);

impl<A: ToString + Clone, DB: ToLiteral> HasValue<A, DB> for CompositKey<A> {
    fn to_sql(&self) -> String where Self: Sized {
        DB::to_literal(self.0.clone())
    }
}

impl<A: ToString> ToString for CompositKey<A> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// Expr (ValueList a)
pub trait HasValueList<A>: ToString {
    fn is_empty(&self) -> bool;
}

pub enum List<A, DB> {
    NonEmpty(Box<dyn HasValue<A, DB>>),
    Empty,
}

impl<A: ToString, DB> HasValueList<A> for List<A, DB> {
    fn is_empty(&self) -> bool {
        match self {
            List::NonEmpty(_) => false,
            List::Empty => true,
        }
    }
}

impl<A, DB> ToString for List<A, DB> {
    fn to_string(&self) -> String {
        match self {
            List::NonEmpty(a) => a.to_string(),
            List::Empty => String::default(),
        }
    }
}

// Expr (OrderBy)
pub trait HasOrder: ToString {}

pub struct OrderBy<A, DB>(pub OrderByType, pub Rc<HasValue<A, DB>>);

impl<A, DB> HasOrder for OrderBy<A, DB> {}

impl<A, DB> ToString for OrderBy<A, DB> {
    fn to_string(&self) -> String {
        let typ = match self.0 {
            OrderByType::Asc => " ASC".to_string(),
            OrderByType::Desc => " DESC".to_string(),
        };
        self.1.to_string() + &typ
    }
}

pub type OrderClause = Rc<HasOrder>;

#[derive(Clone)]
pub enum FromClause {
    Start(String),
    Join(Rc<FromClause>, JoinKind, Rc<FromClause>, Option<Rc<HasValue<bool, bool>>>),
    OnClause(Rc<HasValue<bool, bool>>),
}

impl ToString for FromClause {
    fn to_string(&self) -> String {
        match self {
            FromClause::Start(ref s) => s.to_string(),
            FromClause::Join(lhs, kind, rhs, ref on) => {
                let s = on.clone().unwrap();
                format!("{} {} {} ON {}", lhs.to_string(), kind.to_string(), rhs.to_string(), s.to_string())
            }
            FromClause::OnClause(_) => String::default(),
        }
    }
}

impl FromClause {
    pub fn on(self, on: Rc<HasValue<bool, bool>>) -> Option<FromClause> {
        match self {
            FromClause::Join(lhs, knd, rhs, None) => Some(FromClause::Join(lhs, knd, rhs, Some(on))),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum WhereClause {
    Where(Rc<HasValue<bool, bool>>),
    No,
}

impl Add for WhereClause {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            WhereClause::Where(l) => match other {
                WhereClause::Where(r) => WhereClause::Where(and_(l, r)),
                WhereClause::No => WhereClause::Where(l),
            },
            WhereClause::No => other,
        }
    }
}

impl ToString for WhereClause {
    fn to_string(&self) -> String {
        match self {
            WhereClause::No => String::default(),
            WhereClause::Where(v) => v.to_string(),
        }
    }
}

// Join
#[derive(Debug, Clone)]
pub struct InnerJoin<A, B>(pub A, pub B);

#[derive(Debug, Clone)]
pub struct LeftJoin<A, B>(pub A, pub B);

#[derive(Debug, Clone)]
pub struct RightJoin<A, B>(pub A, pub B);

#[derive(Debug, Clone)]
pub enum JoinKind {
    InnerJoinKind,     // INNER JOIN
    LeftOuterJoinKind, // LEFT OUTER JOIN
    RightOuterJoinKind, // RIGHT OUTER JOIN
}

impl ToString for JoinKind {
    fn to_string(&self) -> String {
        match self {
            JoinKind::InnerJoinKind => String::from("INNER JOIN"),
            JoinKind::LeftOuterJoinKind => String::from("LEFT OUTER JOIN"),
            JoinKind::RightOuterJoinKind => String::from("RIGHT OUTER JOIN"),
        }
    }
}

#[derive(Clone)]
pub struct QueryState {
    pub from_clause: Vec<FromClause>,
    pub where_clause: WhereClause,
    pub order_clause: Vec<OrderClause>,
}

impl Default for QueryState {
    fn default() -> Self {
        QueryState {
            from_clause: vec![],
            order_clause: vec![],
            where_clause: WhereClause::No,
        }
    }
}

impl Add for QueryState {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.from_clause.append(&mut other.from_clause.clone());
        self.where_clause = other.where_clause;
        self
    }
}
