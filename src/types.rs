use std::default::Default;
use std::ops::Add;
use std::rc::Rc;
use std::fmt;

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
    fn to_literal<A: fmt::Display>(v: A) -> String ;
}

impl ToLiteral for String {
    fn to_literal<A: fmt::Display>(v: A) -> String {
        format!("'{}'", v.to_string()) 
    }
}

impl ToLiteral for bool {
    fn to_literal<A: fmt::Display>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for i32 {
    fn to_literal<A: fmt::Display>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for u32 {
    fn to_literal<A: fmt::Display>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

impl ToLiteral for Column {
    fn to_literal<A: fmt::Display>(v: A) -> String {
        format!("{}", v.to_string()) 
    }
}

// Expr (Value a)
pub trait HasValue<A, DB> : fmt::Display {
    fn to_sql(&self) -> String;
}

impl<A: std::fmt::Display, DB> HasValue<A, DB> for Rc<HasValue<A, DB>> where Self: fmt::Display {
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

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            NeedParens::Never => write!(f, "{}", self.1),
            NeedParens::Parens => write!(f, "({})", self.1),
        }
    }
}

pub struct CompositKey<A: fmt::Display>(pub A);

impl<A: fmt::Display + Clone, DB: ToLiteral> HasValue<A, DB> for CompositKey<A> {
    fn to_sql(&self) -> String where Self: Sized {
        DB::to_literal(self.0.clone())
    }
}

impl<A: fmt::Display> fmt::Display for CompositKey<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Expr (ValueList a)
pub trait HasValueList<A>: fmt::Display {
    fn is_empty(&self) -> bool;
}

pub enum List<A, DB> {
    NonEmpty(Box<dyn HasValue<A, DB>>),
    Empty,
}

impl<A: fmt::Display, DB> HasValueList<A> for List<A, DB> {
    fn is_empty(&self) -> bool {
        match self {
            List::NonEmpty(_) => false,
            List::Empty => true,
        }
    }
}

impl<A, DB> fmt::Display for List<A, DB> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::NonEmpty(a) => write!(f, "{}", a),
            List::Empty => write!(f, "{}", String::default()),
        }
    }
}

// Expr (OrderBy)
pub trait HasOrder: fmt::Display {}

pub struct OrderBy<A, DB>(pub OrderByType, pub Rc<HasValue<A, DB>>);

impl<A, DB> HasOrder for OrderBy<A, DB> {}

impl<A, DB> fmt::Display for OrderBy<A, DB> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match self.0 {
            OrderByType::Asc => "ASC",
            OrderByType::Desc => "DESC",
        };
        write!(f, "{} {}", self.1, typ)
    }
}

pub type OrderClause = Rc<HasOrder>;

#[derive(Clone)]
pub enum FromClause {
    Start(String),
    Join(Rc<FromClause>, JoinKind, Rc<FromClause>, Option<Rc<HasValue<bool, bool>>>),
    OnClause(Rc<HasValue<bool, bool>>),
}

impl fmt::Display for FromClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FromClause::Start(s) => write!(f, "{}", s),
            FromClause::Join(lhs, kind, rhs, ref on) if on.is_some() => 
                write!(f, "{} {} {} ON {}", lhs, kind, rhs, on.clone().unwrap()),
            _ => Ok(()),
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

impl fmt::Display for WhereClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhereClause::No => Ok(()),
            WhereClause::Where(v) => write!(f, "{}", v),
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

impl fmt::Display for JoinKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            JoinKind::InnerJoinKind => "INNER JOIN",
            JoinKind::LeftOuterJoinKind => "LEFT OUTER JOIN",
            JoinKind::RightOuterJoinKind => "RIGHT OUTER JOIN",
        };
        write!(f, "{}", kind)
    }
}

pub trait HasSet : fmt::Display {}

pub struct SetValue<A, DB1, DB2>(pub Rc<HasValue<A, DB1>>, pub Rc<HasValue<A, DB2>>);

impl<A, DB1, DB2> HasSet for SetValue<A, DB1, DB2> {}

impl<A, DB1, DB2> fmt::Display for SetValue<A, DB1, DB2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.0, self.1.to_sql())
    }
}

pub type SetClause = Rc<HasSet>;

#[derive(Clone)]
pub struct QueryState {
    pub from_clause: Vec<FromClause>,
    pub where_clause: WhereClause,
    pub order_clause: Vec<OrderClause>,
    pub set_clause: Vec<SetClause>,
}

impl Default for QueryState {
    fn default() -> Self {
        QueryState {
            from_clause: vec![],
            order_clause: vec![],
            where_clause: WhereClause::No,
            set_clause: vec![],
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
