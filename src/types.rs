use std::default::Default;
use std::ops::Add;
use std::rc::Rc;

use entity::Entity;
use expression::*;

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

// Expr (Value a)
pub trait HasValue<A>: ToString {}

#[derive(Clone)]
pub enum NeedParens {
    Never,
    Parens,
}

#[derive(Clone)]
pub struct Raw(pub NeedParens, pub String);

impl<A> HasValue<A> for Raw {}

impl ToString for Raw {
    fn to_string(&self) -> String {
        match self.0 {
            NeedParens::Never => self.1.clone(),
            NeedParens::Parens => "(".to_owned() + &self.1 + ")",
        }
    }
}

pub struct CompositKey<A>(pub A);

impl<A: ToString> HasValue<A> for CompositKey<A> {}

impl<A: ToString> ToString for CompositKey<A> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// Expr (ValueList a)
pub trait HasValueList<A>: ToString {
    fn is_empty(&self) -> bool;
}

pub enum List<A> {
    NonEmpty(Box<dyn HasValue<A>>),
    Empty,
}

impl<A: ToString> HasValueList<A> for List<A> {
    fn is_empty(&self) -> bool {
        match self {
            List::NonEmpty(_) => false,
            List::Empty => true,
        }
    }
}

impl<A> ToString for List<A> {
    fn to_string(&self) -> String {
        match self {
            List::NonEmpty(a) => a.to_string(),
            List::Empty => String::default(),
        }
    }
}

// Expr (OrderBy)
pub trait HasOrder: ToString {}

pub struct OrderBy<A>(pub OrderByType, pub Rc<HasValue<A>>);

impl<A> HasOrder for OrderBy<A> {}

impl<A> ToString for OrderBy<A> {
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
    Join(Rc<FromClause>, JoinKind, Rc<FromClause>, Option<Rc<HasValue<bool>>>),
    OnClause(Rc<HasValue<bool>>),
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
    pub fn on(self, on: Rc<HasValue<bool>>) -> Option<FromClause> {
        match self {
            FromClause::Join(lhs, knd, rhs, None) => Some(FromClause::Join(lhs, knd, rhs, Some(on))),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum WhereClause {
    Where(Rc<HasValue<bool>>),
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

impl From<WhereClause> for String {
    fn from(inst: WhereClause) -> String {
        match inst {
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
