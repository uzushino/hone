use std::rc::Rc;

use entity::*;
use query::*;

impl<A> Query<A> {
    fn new(e: A) -> Self {
        Query {
            state: QueryState::default(),
            value: e,
        }
    }

    pub fn return_<B>(self, ret: B) -> Query<B> {
        let mut q = Query::new(ret);
        q.state = self.state.clone();
        q
    }

    pub fn on_(mut self, b: Rc<HasValue<bool>>) -> Query<A> {
        self.state.from_clause.push(FromClause::OnClause(b));
        self
    }

    pub fn where_(mut self, b: Rc<HasValue<bool>>) -> Query<A> {
        let w = WhereClause::Where(b);
        self.state.where_clause = self.state.where_clause + w;
        self
    }

    pub fn order_(mut self, b: Vec<Rc<HasOrder>>) -> Query<A> {
        self.state.order_clause = b;
        self
    }

    fn from_start() -> FromPreprocess<A>
    where
        A: Default + HasEntityDef,
    {
        let from_ = FromClause::Start(A::entity_def().table_name);
        FromPreprocess(A::default(), from_)
    }

    fn from_finish(q: &mut Query<A>, exp: FromPreprocess<A>) -> Result<A, ()> {
        q.state.from_clause.push(exp.1);
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
        let mut qs = Query::new((a.value, b.value));

        qs.state = a.state + b.state;

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
