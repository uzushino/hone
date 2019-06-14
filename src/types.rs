use std::default::Default;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;

use crate::entity::{Column, Entity};
use crate::expression::and_;
use crate::query::ToValues;

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
    fn to_literal<A: fmt::Display>(v: A) -> String;
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
pub trait HasValue<A, DB>: fmt::Display {
    fn to_sql(&self) -> String;
}

#[derive(Clone)]
pub enum NeedParens {
    Never,
    Parens,
}

#[derive(Clone)]
pub struct Raw(pub NeedParens, pub String);

impl<A, DB: ToLiteral> HasValue<A, DB> for Raw {
    fn to_sql(&self) -> String
    where
        Self: Sized,
    {
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
    fn to_sql(&self) -> String
    where
        Self: Sized,
    {
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
            FromClause::Join(lhs, kind, rhs, ref on) if on.is_some() => write!(f, "{} {} {} ON {}", lhs, kind, rhs, on.clone().unwrap()),
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
    InnerJoinKind,      // INNER JOIN
    LeftOuterJoinKind,  // LEFT OUTER JOIN
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

// SET

pub trait HasSet: fmt::Display {
    fn column(&self) -> String;
    fn value(&self) -> String;
}

pub struct SetValue<A, DB>(pub Rc<HasValue<A, Column>>, pub Rc<HasValue<A, DB>>);

impl<A, DB> HasSet for SetValue<A, DB> {
    fn column(&self) -> String {
        self.0.to_sql()
    }

    fn value(&self) -> String {
        self.1.to_sql()
    }
}

impl<A, DB2> fmt::Display for SetValue<A, DB2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.0, self.1.to_sql())
    }
}

pub type SetClause = Box<HasSet>;

// LIMIT / OFFSET

#[derive(Clone)]
pub enum LimitClause {
    Limit(Option<u32>, Option<u32>),
    No,
}

impl Default for LimitClause {
    fn default() -> Self {
        LimitClause::No
    }
}

impl Add for LimitClause {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            LimitClause::Limit(lhs1, rhs1) => match other {
                LimitClause::Limit(lhs2, rhs2) => LimitClause::Limit(lhs1.or(lhs2), rhs1.or(rhs2)),
                LimitClause::No => self,
            },
            LimitClause::No => other,
        }
    }
}

impl fmt::Display for LimitClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LimitClause::Limit(limit, offset) => {
                let mut result: Vec<String> = Vec::new();

                if let Some(n) = limit {
                    result.push(format!("LIMIT {}", n));
                }
                if let Some(n) = offset {
                    result.push(format!("OFFSET {}", n));
                }

                write!(f, "{}", result.join(" "))
            }
            LimitClause::No => Err(fmt::Error),
        }
    }
}

// GROUP BY

pub trait HasGroupBy: fmt::Display {}

pub struct GroupBy<A, DB>(pub Rc<HasValue<A, DB>>);

impl<A, DB> HasGroupBy for GroupBy<A, DB> {}

pub type GroupByClause = Box<HasGroupBy>;

impl<A, DB> fmt::Display for GroupBy<A, DB> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// DISTNCT(ON)
pub trait HasDistinct: fmt::Display {
    fn box_clone(&self) -> Box<HasDistinct>;
}

impl Clone for Box<HasDistinct> {
    fn clone(&self) -> Box<HasDistinct> {
        self.box_clone()
    }
}

impl<A, DB> HasDistinct for Rc<HasValue<A, DB>>
where
    A: 'static,
    DB: 'static,
{
    fn box_clone(&self) -> Box<HasDistinct> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub enum Distinct {
    All,
    Standard,
    On(Vec<Box<dyn HasDistinct>>),
}

impl HasDistinct for Distinct {
    fn box_clone(&self) -> Box<HasDistinct> {
        Box::new((*self).clone())
    }
}

impl Default for Distinct {
    fn default() -> Self {
        Distinct::All
    }
}

impl fmt::Display for Distinct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Distinct::All => write!(f, ""),
            Distinct::Standard => write!(f, "DISTINCT "),
            Distinct::On(vs) => {
                let cs = vs.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "DISTINCT ON ({}) ", cs)
            }
        }
    }
}

pub type DistinctClause = Distinct;

pub type ValuesClause = Box<HasValues>;

pub struct QueryState {
    pub distinct_clause: DistinctClause,
    pub from_clause: Vec<FromClause>,
    pub where_clause: WhereClause,
    pub order_clause: Vec<OrderClause>,
    pub set_clause: Vec<SetClause>,
    pub values_clause: Option<ValuesClause>,
    pub limit_clause: LimitClause,
    pub groupby_clause: Vec<GroupByClause>,
    pub having_clause: WhereClause,
}

impl Default for QueryState {
    fn default() -> Self {
        QueryState {
            distinct_clause: DistinctClause::default(),
            from_clause: vec![],
            order_clause: vec![],
            where_clause: WhereClause::No,
            set_clause: vec![],
            values_clause: None,
            limit_clause: LimitClause::default(),
            groupby_clause: vec![],
            having_clause: WhereClause::No,
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

pub trait HasValues {
    fn columns(&self) -> Vec<String> {
        vec![]
    }

    fn values(&self) -> Vec<Vec<String>> {
        vec![]
    }
}

pub struct Values<A: ToValues, B: ToValues>(pub A, pub Vec<B>);

impl<A: ToValues, B: ToValues> HasValues for Values<A, B> {
    fn columns(&self) -> Vec<String> {
        self.0.to_vec()
    }

    fn values(&self) -> Vec<Vec<String>> {
        self.1
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>()
    }
}