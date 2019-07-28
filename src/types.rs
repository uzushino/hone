use std::default::Default;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;

use crate::entity::{ Column, Entity };
use crate::expression::and_;
//use crate::query::ToValues;

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
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String;
}

impl ToLiteral for String {
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String {
        format!("'{}'", v.to_string())
    }
}

impl ToLiteral for bool {
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String {
        format!("{}", v.to_string())
    }
}

impl ToLiteral for i32 {
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String {
        format!("{}", v.to_string())
    }
}

impl ToLiteral for u32 {
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String {
        format!("{}", v.to_string())
    }
}

impl ToLiteral for Column {
    fn to_literal<'a, A: fmt::Display>(v: &'a A) -> String {
        format!("{}", v.to_string())
    }
}

// Expr (Value a)
pub trait HasValue<A>: fmt::Display {
    type Output;

    fn to_sql(&self) -> String;
}

pub type SqlExpr<A, B> = Rc<HasValue<A, Output=B>>;

#[derive(Clone)]
pub enum NeedParens {
    Never,
    Parens,
}

#[derive(Clone)]
pub struct Raw<A>(pub NeedParens, pub String, pub std::marker::PhantomData<A>);

impl<A, B> HasValue<A> for Raw<B> where Self: Sized, B: ToLiteral {
    type Output = B;

    fn to_sql(&self) -> String where Self: Sized, <Self as HasValue<A>>::Output: ToLiteral {
        let s = Self::Output::to_literal(&self.1);

        match self.0 {
            NeedParens::Never => s.to_string(),
            NeedParens::Parens => "(".to_owned() + s.as_str() + ")",
        }
    }
}

impl<A> fmt::Display for Raw<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            NeedParens::Never => write!(f, "{}", self.1),
            NeedParens::Parens => write!(f, "({})", self.1),
        }
    }
}

pub struct CompositKey<A>(pub A);

impl<A: fmt::Display + Clone + ToLiteral> HasValue<A> for CompositKey<A> {
    type Output = A;

    fn to_sql(&self) -> String where Self: Sized {
        Self::Output::to_literal(&self.0)
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

pub enum List<A, B: ToLiteral> {
    NonEmpty(Box<dyn HasValue<A, Output=B>>),
    Empty,
}

impl<A: fmt::Display, B: ToLiteral> HasValueList<A> for List<A, B> {
    fn is_empty(&self) -> bool {
        match self {
            List::NonEmpty(_) => false,
            List::Empty => true,
        }
    }
}

impl<A, B: ToLiteral> fmt::Display for List<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::NonEmpty(a) => write!(f, "{}", a),
            List::Empty => write!(f, "{}", String::default()),
        }
    }
}

// Expr (OrderBy)
pub trait HasOrder: fmt::Display {}

pub struct OrderBy<A, B>(pub OrderByType, pub Box<HasValue<A, Output=B>>);

impl<A, B: ToLiteral> HasOrder for OrderBy<A, B> {}

impl<A, B: ToLiteral> fmt::Display for OrderBy<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match self.0 {
            OrderByType::Asc => "ASC",
            OrderByType::Desc => "DESC",
        };

        write!(f, "{} {}", self.1.to_sql(), typ)
    }
}

pub type OrderClause = Box<HasOrder>;

#[derive(Clone)]
pub enum FromClause {
    Start(String),
    Join(Box<FromClause>, JoinKind, Box<FromClause>, Option<Rc<Box<HasValue<bool, Output=bool>>>>),
    OnClause(Rc<Box<HasValue<bool, Output=bool>>>),
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
    pub fn on(self, on: Box<HasValue<bool, Output=bool>>) -> Option<FromClause> {
        match self {
            FromClause::Join(lhs, knd, rhs, None) => Some(FromClause::Join(lhs, knd, rhs, Some(Rc::new(on)))),
            _ => None,
        }
    }
}

pub enum WhereClause<'a> {
    Where(Box<'a + HasValue<bool, Output=bool>>),
    No,
}

impl<'a> Add for WhereClause<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            WhereClause::Where(ref l) => match other {
                WhereClause::Where(ref r) => WhereClause::Where(and_(l.as_ref(), r.as_ref())),
                _ => unimplemented!()
            },
            WhereClause::No => other,
        }
    }
}

impl<'a> fmt::Display for WhereClause<'a> {
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

pub struct SetValue<A, B>(
    pub Box<HasValue<A, Output=Column>>, 
    pub Box<HasValue<A, Output=B>>
);

impl<A, B: ToLiteral> HasSet for SetValue<A, B> {
    fn column(&self) -> String {
        self.0.to_sql()
    }

    fn value(&self) -> String {
        self.1.to_sql()
    }
}

impl<A, B: ToLiteral> fmt::Display for SetValue<A, B> {
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

pub struct GroupBy<A, B>(pub Rc<HasValue<A, Output=B>>);

impl<A, B> HasGroupBy for GroupBy<A, B> {}

pub type GroupByClause = Box<HasGroupBy>;

impl<A, B> fmt::Display for GroupBy<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// DISTNCT(ON)
pub trait HasDistinct: fmt::Display {
}

impl<A, B> HasDistinct for Box<HasValue<A, Output=B>> where A: 'static, B: 'static {}

pub enum Distinct {
    All,
    Standard,
    On(Vec<Box<dyn HasDistinct>>),
}

impl HasDistinct for Distinct {}

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

pub struct QueryState<'a> {
    pub distinct_clause: DistinctClause,
    pub from_clause: Vec<FromClause>,
    pub where_clause: WhereClause<'a>,
    pub order_clause: Vec<OrderClause>,
    pub set_clause: Vec<SetClause>,
    pub values_clause: Option<ValuesClause>,
    pub limit_clause: LimitClause,
    pub groupby_clause: Vec<GroupByClause>,
    pub having_clause: WhereClause<'a>,
}

impl<'a> Default for QueryState<'a> {
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

impl<'a> Add for QueryState<'a> {
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

/*
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
*/