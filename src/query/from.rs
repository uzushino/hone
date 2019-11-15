use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::Column as CL;
use crate::entity::*;
use crate::query::*;
use crate::types::Values;

impl<A> Query<A> {
    pub fn new(e: A) -> Self {
        Query {
            state: Rc::new(RefCell::new(QueryState::default())),
            value: e,
        }
    }

    pub fn return_<B>(self, ret: B) -> Query<B> {
        let mut q = Query::new(ret);
        q.state = self.state;
        q
    }

    pub fn on_(self, b: Rc<dyn HasValue<bool, Output = bool>>) -> Query<A> {
        self.state.borrow_mut().from_clause.push(FromClause::OnClause(b));
        self
    }

    pub fn where_(self, b: Rc<dyn HasValue<bool, Output = bool>>) -> Query<A> {
        let w = WhereClause::Where(b);
        let mut s = self.state.borrow_mut().where_clause.add(w);
        std::mem::swap(&mut s, &mut self.state.borrow_mut().where_clause);
        self
    }

    pub fn order_(self, b: Vec<Rc<dyn HasOrder>>) -> Query<A> {
        self.state.borrow_mut().order_clause = b;
        self
    }

    pub fn group_by_<T, DB: ToLiteral>(self, b: Rc<dyn HasValue<T, Output = DB>>) -> Query<A>
    where
        T: 'static,
        DB: 'static,
    {
        let v = GroupBy(b);
        self.state.borrow_mut().groupby_clause.push(Box::new(v));
        self
    }

    pub fn having_(self, b: Rc<dyn HasValue<bool, Output = bool>>) -> Query<A> {
        let w = WhereClause::Where(b);
        let n = self.state.borrow_mut().having_clause.clone();

        {
            (*self.state.borrow_mut()).having_clause = n.add(w);
        }

        self
    }

    pub fn value_<T, DB: ToLiteral>(self, a: Rc<dyn HasValue<T, Output = CL>>, b: Rc<dyn HasValue<T, Output = DB>>) -> Query<A>
    where
        T: 'static,
        DB: 'static,
    {
        let v = Box::new(SetValue(a, b));
        self.state.borrow_mut().set_clause.push(v);
        self
    }

    pub fn values_<T, S>(self, a: T, b: Vec<S>) -> Query<A>
    where
        T: ToValues + 'static,
        S: ToValues + 'static,
    {
        self.state.borrow_mut().values_clause = Some(Box::new(Values(a, b)));
        self
    }

    pub fn limit_(self, a: u32) -> Query<A> {
        let s = self.state.borrow_mut().limit_clause.clone();
        {
            self.state.borrow_mut().limit_clause = s + LimitClause::Limit(Some(a), None);
        }
        self
    }

    pub fn offset_(self, a: u32) -> Query<A> {
        let s = self.state.borrow_mut().limit_clause.clone();
        {
            self.state.borrow_mut().limit_clause = s + LimitClause::Limit(None, Some(a));
        }
        self
    }

    pub fn distinct_on_(self, mut a: Vec<Box<dyn HasDistinct>>) -> Query<A> {
        {
            let s = &mut *self.state.borrow_mut();

            match &mut s.distinct_clause {
                Distinct::On(ref mut v) => {
                    v.append(&mut a);
                    s.distinct_clause = Distinct::On(v.to_vec())
                }
                Distinct::All => s.distinct_clause = Distinct::On(a),
                _ => {}
            };
        }

        self
    }

    pub fn dup_key_<S, T>(self, column: Rc<dyn HasValue<S, Output = CL>>, value: Rc<dyn HasValue<S, Output = T>>) -> Query<A>
    where
        S: 'static,
        T: 'static,
    {
        {
            let a = DuplicateKey(column, value);
            self.state.borrow_mut().duplicate_clause.push(Box::new(a));
        }
        self
    }

    fn from_start() -> FromPreprocess<A>
    where
        A: Default + HasEntityDef,
    {
        let from_ = FromClause::Start(A::table_name().name());
        FromPreprocess(A::default(), from_)
    }

    fn from_finish(q: &mut Query<A>, exp: FromPreprocess<A>) -> Result<A, ()> {
        q.state.borrow_mut().from_clause.push(exp.1);
        Ok(exp.0)
    }
}

trait IsJoin<A, B> {
    type Kind;

    fn smart_join(lhs: A, rhs: B) -> Self::Kind;
    fn from_join(lhs: FromPreprocess<A>, rhs: FromPreprocess<B>) -> Result<FromPreprocess<Self::Kind>, ()>;
}

impl<A, B> IsJoin<A, B> for InnerJoin<A, B> {
    type Kind = InnerJoin<A, B>;

    fn smart_join(lhs: A, rhs: B) -> Self::Kind {
        InnerJoin(lhs, rhs)
    }

    fn from_join(lhs: FromPreprocess<A>, rhs: FromPreprocess<B>) -> Result<FromPreprocess<Self::Kind>, ()> {
        fn get_process<T>(p: FromPreprocess<T>) -> Result<(T, FromClause), ()> {
            Ok((p.0, p.1))
        };

        let (l1, lf) = get_process(lhs)?;
        let (r1, rf) = get_process(rhs)?;

        let join_ = InnerJoin::smart_join(l1, r1);
        let from_ = FromClause::Join(Rc::new(lf), JoinKind::InnerJoinKind, Rc::new(rf), None);

        Ok(FromPreprocess(join_, from_))
    }
}

impl<A, B> IsJoin<A, B> for LeftJoin<A, B> {
    type Kind = LeftJoin<A, B>;

    fn smart_join(lhs: A, rhs: B) -> Self::Kind {
        LeftJoin(lhs, rhs)
    }

    fn from_join(lhs: FromPreprocess<A>, rhs: FromPreprocess<B>) -> Result<FromPreprocess<Self::Kind>, ()> {
        fn get_process<T>(p: FromPreprocess<T>) -> Result<(T, FromClause), ()> {
            Ok((p.0, p.1))
        };

        let (l1, lf) = get_process(lhs)?;
        let (r1, rf) = get_process(rhs)?;
        let join_ = LeftJoin::smart_join(l1, r1);
        let from_ = FromClause::Join(Rc::new(lf), JoinKind::LeftOuterJoinKind, Rc::new(rf), None);

        Ok(FromPreprocess(join_, from_))
    }
}

impl<A, B> IsJoin<A, B> for RightJoin<A, B> {
    type Kind = RightJoin<A, B>;

    fn smart_join(lhs: A, rhs: B) -> Self::Kind {
        RightJoin(lhs, rhs)
    }

    fn from_join(lhs: FromPreprocess<A>, rhs: FromPreprocess<B>) -> Result<FromPreprocess<Self::Kind>, ()> {
        fn get_process<T>(p: FromPreprocess<T>) -> Result<(T, FromClause), ()> {
            Ok((p.0, p.1))
        };

        let (l1, lf) = get_process(lhs)?;
        let (r1, rf) = get_process(rhs)?;
        let join_ = RightJoin::smart_join(l1, r1);
        let from_ = FromClause::Join(Rc::new(lf), JoinKind::RightOuterJoinKind, Rc::new(rf), None);

        Ok(FromPreprocess(join_, from_))
    }
}

impl<A> Query<Option<A>> {
    fn from_option() -> FromPreprocess<Option<A>>
    where
        A: Default + HasEntityDef,
    {
        let a = Query::<A>::from_start();
        FromPreprocess(Some(a.0), a.1)
    }
}

impl<A> FromQuery for Query<A>
where
    A: Default + HasQuery<T = A> + FromProcess<Item = A>,
{
    type Kind = A;

    fn from_() -> Result<Query<A>, ()> {
        let mut qs = Query::new(A::default());
        let s = A::from_process()?;
        let _ = Query::<A>::from_finish(&mut qs, s);

        Ok(qs)
    }

    fn from_by<F, R>(f: F) -> Result<Query<R>, ()>
    where
        F: Fn(Query<Self::Kind>, Self::Kind) -> Query<R>,
    {
        let qs = Query::<Self::Kind>::from_()?;

        Ok(f(qs, Self::Kind::default()))
    }
}

impl<A> FromQuery for Query<Option<A>>
where
    A: Default + HasEntityDef + HasQuery<T = A>,
{
    type Kind = Option<A>;

    fn from_() -> Result<Query<Self::Kind>, ()> {
        let mut qs = Query::new(Option::<A>::default());
        let s = Option::<A>::from_process()?;
        let _ = Query::<Option<A>>::from_finish(&mut qs, s);

        Ok(qs)
    }

    fn from_by<F, R>(f: F) -> Result<Query<R>, ()>
    where
        F: Fn(Query<Self::Kind>, Self::Kind) -> Query<R>,
    {
        let qs = Query::<Self::Kind>::from_()?;

        Ok(f(qs, Self::Kind::default()))
    }
}

impl<A, B> FromQuery for Query<(A, B)>
where
    A: Default + FromProcess<Item = A> + HasQuery<T = A>,
    B: Default + FromProcess<Item = B> + HasQuery<T = B>,
{
    type Kind = (A, B);

    fn from_() -> Result<Query<Self::Kind>, ()> {
        let a = Query::<A>::from_()?;
        let b = Query::<B>::from_()?;
        let qs = Query::new((a.value, b.value));

        {
            let mut sa = a.state.borrow_mut();
            let mut sb = b.state.borrow_mut();
            sa.from_clause.append(&mut sb.from_clause);

            let mut s = QueryState {
                where_clause: sa.where_clause.add(sb.where_clause.clone()),
                ..Default::default()
            };

            std::mem::replace(&mut s.from_clause, (* sa.from_clause).to_vec());

            qs.state.replace(s);
        }

        Ok(qs)
    }

    fn from_by<F, R>(f: F) -> Result<Query<R>, ()>
    where
        F: Fn(Query<Self::Kind>, Self::Kind) -> Query<R>,
    {
        let qs = Query::<Self::Kind>::from_()?;

        Ok(f(qs, Self::Kind::default()))
    }
}

impl<A, B> Default for InnerJoin<A, B>
where
    A: Default + HasQuery<T = A>,
    B: Default + HasQuery<T = B>,
{
    fn default() -> Self {
        InnerJoin(A::default(), B::default())
    }
}

impl<A, B> HasQuery for InnerJoin<A, B> {
    type T = InnerJoin<A, B>;
}

impl<A, B> Default for LeftJoin<A, B>
where
    A: Default + HasQuery<T = A>,
    B: Default + HasQuery<T = B>,
{
    fn default() -> Self {
        LeftJoin(A::default(), B::default())
    }
}

impl<A, B> HasQuery for LeftJoin<A, B> {
    type T = LeftJoin<A, B>;
}

impl<A, B> Default for RightJoin<A, B>
where
    A: Default + HasQuery<T = A>,
    B: Default + HasQuery<T = B>,
{
    fn default() -> Self {
        RightJoin(A::default(), B::default())
    }
}

impl<A, B> HasQuery for RightJoin<A, B> {
    type T = RightJoin<A, B>;
}

pub trait FromProcess {
    type Item;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()>;
}

impl<A> FromProcess for A
where
    A: Default + HasEntityDef + HasQuery<T = A>,
{
    type Item = A;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()> {
        Ok(Query::<Self::Item>::from_start())
    }
}

impl<A> FromProcess for Option<A>
where
    A: Default + HasEntityDef + HasQuery<T = A>,
{
    type Item = Option<A>;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()> {
        Ok(Query::<Self::Item>::from_option())
    }
}

impl<A, B> FromProcess for InnerJoin<A, B>
where
    A: Default + HasQuery<T = A> + FromProcess<Item = A>,
    B: Default + HasQuery<T = B> + FromProcess<Item = B>,
{
    type Item = InnerJoin<A, B>;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()> {
        let lhs = A::from_process()?;
        let rhs = B::from_process()?;

        InnerJoin::<A, B>::from_join(lhs, rhs)
    }
}

impl<A, B> FromProcess for LeftJoin<A, B>
where
    A: Default + HasQuery<T = A> + FromProcess<Item = A>,
    B: Default + HasQuery<T = B> + FromProcess<Item = B>,
{
    type Item = LeftJoin<A, B>;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()> {
        let lhs = A::from_process()?;
        let rhs = B::from_process()?;

        LeftJoin::<A, B>::from_join(lhs, rhs)
    }
}

impl<A, B> FromProcess for RightJoin<A, B>
where
    A: Default + HasQuery<T = A> + FromProcess<Item = A>,
    B: Default + HasQuery<T = B> + FromProcess<Item = B>,
{
    type Item = RightJoin<A, B>;

    fn from_process() -> Result<FromPreprocess<Self::Item>, ()> {
        let lhs = A::from_process()?;
        let rhs = B::from_process()?;

        RightJoin::<A, B>::from_join(lhs, rhs)
    }
}

pub fn set_on(join: &FromClause, on: &Rc<dyn HasValue<bool, Output = bool>>) -> Option<FromClause> {
    match join {
        FromClause::Join(lhs, _, rhs, on_) => {
            if let Some(f) = set_on(rhs.borrow(), on) {
                return Some(join.clone().set_rhs(f));
            }
            if let Some(f) = set_on(lhs.borrow(), on) {
                return Some(join.clone().set_lhs(f));
            }
            match on_ {
                None => Some(join.clone().set_on(on.clone())),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn find_imcomplete_and_set_on(
    joins: &[FromClause],
    on: &Rc<dyn HasValue<bool, Output = bool>>,
) -> Result<Vec<FromClause>, Rc<dyn HasValue<bool, Output = bool>>> {
    match joins.split_first() {
        Some((ref join, rest)) => {
            if let Some(f) = set_on(*join, &on) {
                let mut rest = rest.to_vec();
                rest.push(f);
                return Ok(rest);
            }

            let mut v = find_imcomplete_and_set_on(rest, on)?;
            v.insert(0, (*join).clone());

            Ok(v)
        }
        None => Err(on.clone()),
    }
}

pub fn combine_joins(fs: &[FromClause], acc: &mut [FromClause]) -> Result<Vec<FromClause>, ()> {
    match fs.split_first() {
        Some((FromClause::OnClause(on), rest)) => {
            match find_imcomplete_and_set_on(acc, on) {
                Ok(mut acc_) => combine_joins(rest, acc_.as_mut_slice()),
                Err(_) => Err(()),
            }
        }
        Some((head, rest)) => {
            let mut acc = acc.to_vec();
            acc.push(head.clone());

            combine_joins(rest, acc.as_mut_slice())
        }
        _ => {
            acc.reverse();
            Ok(acc.to_vec())
        }
    }
}
