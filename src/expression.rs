use std::fmt;
use std::rc::Rc;

use crate::entity::*;
use crate::query::*;
use crate::types::*;

pub fn parens_<'a, A, B, C>(a: A) -> Rc<dyn 'a + HasValue<B, Output = C>>
where
    A: Into<String>,
    C: 'a + ToLiteral,
{
    Rc::new(Raw(NeedParens::Parens, a.into(), std::marker::PhantomData))
}

pub fn never_<'a, A, B, C>(a: A) -> Rc<dyn 'a + HasValue<B, Output = C>>
where
    A: Into<String>,
    C: 'a + ToLiteral,
{
    Rc::new(Raw(NeedParens::Never, a.into(), std::marker::PhantomData))
}

pub fn star_<A: HasEntityDef>() -> Rc<dyn HasValue<Star, Output = Column>> {
    let t = A::table_name();
    Rc::new(Column::new(format!("{}.{}", t.name(), "*").as_str()))
}

pub fn eq_<A, B, C>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    A: ToLiteral,
    B: ToLiteral,
{
    let a = lhs.to_sql();
    let b = rhs.to_sql();

    parens_(a + " = " + &b)
}

pub fn not_eq_<A, B, C>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    A: ToLiteral,
    B: ToLiteral,
{
    let a = lhs.to_sql();
    let b = rhs.to_sql();

    parens_(a + " <> " + &b)
}

fn if_not_empty_list<A>(v: impl HasValueList<A>, b: bool, e: Rc<dyn HasValue<bool, Output = bool>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    if v.is_empty() {
        return val_(b);
    }
    e
}

pub fn in_<A, B>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: impl HasValueList<A>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    A: ToLiteral,
{
    let comp: Rc<dyn HasValue<A, Output = i32>> = parens_(rhs.to_string());
    if_not_empty_list(rhs, false, binop_(" IN ", &lhs, &comp))
}

pub fn not_in_<A, B>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: impl HasValueList<A>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    A: ToLiteral,
{
    let comp: Rc<dyn HasValue<A, Output = i32>> = parens_(rhs.to_string());
    if_not_empty_list(rhs, false, binop_(" NOT IN ", &lhs, &comp))
}

pub fn val_<'a, A>(typ: A) -> Rc<dyn 'a + HasValue<A, Output = A>>
where
    A: 'a + fmt::Display + ToLiteral,
{
    never_(typ.to_string())
}

pub fn val_list_<'a, A, B>(vs: &[Rc<dyn 'a + HasValue<A, Output = B>>]) -> impl HasValueList<A>
where
    A: 'a + fmt::Display,
    B: 'static + ToLiteral,
{
    if vs.is_empty() {
        return List::Empty as List<A, B>;
    }

    let s = vs.to_vec().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    let v = Raw(NeedParens::Parens, s, std::marker::PhantomData);

    List::NonEmpty(Box::new(v)) as List<A, B>
}

pub fn gt_<A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    binop_(" > ", lhs, rhs)
}

pub fn gte_<A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    binop_(" >= ", lhs, rhs)
}

pub fn lt_<A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    binop_(" < ", lhs, rhs)
}

pub fn lte_<A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    binop_(" <= ", lhs, rhs)
}

pub fn re_<A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    binop_(" ~ ", lhs, rhs)
}

pub fn and_<'a, A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn 'a + HasValue<A, Output = C>>
where
    B: ToLiteral,
    C: 'a + ToLiteral,
{
    binop_(" AND ", lhs, rhs)
}

pub fn or_<'a, A, B, C>(lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn 'a + HasValue<A, Output = C>>
where
    B: ToLiteral,
    C: 'a + ToLiteral,
{
    binop_(" OR ", lhs, rhs)
}

pub fn binop_<'a, A, B, C, D, E>(op: &str, lhs: &Rc<dyn HasValue<A, Output = B>>, rhs: &Rc<dyn HasValue<A, Output = C>>) -> Rc<dyn 'a + HasValue<D, Output = E>>
where
    E: 'a + ToLiteral,
{
    let a = lhs.to_sql();
    let b = rhs.to_sql();

    parens_(a + op + &b)
}

pub fn between_<A, B, C, D>(
    comp: Rc<dyn HasValue<A, Output = B>>,
    lhs: Rc<dyn HasValue<A, Output = C>>,
    rhs: Rc<dyn HasValue<A, Output = D>>,
) -> Rc<dyn HasValue<bool, Output = bool>>
where
    B: ToLiteral,
    C: ToLiteral,
    D: ToLiteral,
{
    let e = comp.to_sql();
    let a = lhs.to_sql();
    let b = rhs.to_sql();

    parens_(e + " BETWEEN " + &a + " TO " + &b)
}

pub fn is_null_<A, B>(a: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    B: ToLiteral,
{
    parens_(a.to_sql() + " IS NULL")
}

pub fn is_not_null_<A, B>(a: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    B: ToLiteral,
{
    parens_(a.to_sql() + " IS NOT NULL")
}

pub fn asc_<'a, A, B>(exp: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn 'a + HasOrder>
where
    A: 'a + fmt::Display,
    B: 'a + ToLiteral,
{
    Rc::new(OrderBy(OrderByType::Asc, exp))
}

pub fn desc_<'a, A, B>(exp: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn 'a + HasOrder>
where
    A: 'a + fmt::Display,
    B: 'a + ToLiteral,
{
    Rc::new(OrderBy(OrderByType::Desc, exp))
}

pub fn exists_<'a, A, B>(q: Query<Rc<dyn HasValue<A, Output = B>>>) -> Rc<dyn HasValue<bool, Output = bool>>
where
    A: fmt::Display,
    B: 'static + ToLiteral,
{
    unsafe_sql_function("EXISTS ", sub_(q), NeedParens::Never)
}

pub fn not_exists_<'a, A, B>(q: Query<Rc<dyn HasValue<A, Output = B>>>) -> Rc<dyn 'a + HasValue<bool, Output = bool>>
where
    A: 'a + fmt::Display,
    B: 'static + ToLiteral,
{
    unsafe_sql_function("NOT EXISTS ", sub_(q), NeedParens::Never)
}

pub fn sub_<'a, A, B>(q: Query<Rc<dyn HasValue<A, Output = B>>>) -> Rc<dyn 'a + HasValue<A, Output = B>>
where
    A: fmt::Display,
    B: 'a + ToLiteral,
{
    parens_(select(q).to_sql())
}

fn unsafe_sql_function<'a, A, B, C>(name: &str, arg: A, parens: NeedParens) -> Rc<dyn 'a + HasValue<B, Output = C>>
where
    A: UnsafeSqlFunctionArgument,
    C: 'a + ToLiteral,
{
    let args = A::to_arg_list(&arg);
    let results = args.iter().map(ToString::to_string).collect::<Vec<_>>();

    let expr = match parens {
        NeedParens::Parens => format!("{}({})", name, results.join(",")),
        NeedParens::Never => format!("{}{}", name, results.join(",")),
    };

    never_(expr)
}

pub fn unsafe_sql_value<'a, A, B>(name: &str) -> Rc<dyn 'a + HasValue<A, Output = B>>
where
    B: 'a + ToLiteral,
{
    never_(name)
}

pub fn random_() -> Rc<dyn HasValue<i32, Output = i32>> {
    unsafe_sql_value("RANDOM()")
}

pub fn count_rows_() -> Rc<dyn HasValue<i32, Output = i32>> {
    unsafe_sql_value("COUNT(*)")
}

pub fn count_columns_<A>(a: Rc<dyn HasValue<A, Output = Column>>) -> Rc<dyn HasValue<i32, Output = i32>> {
    unsafe_sql_function("COUNT", a, NeedParens::Parens)
}

pub fn not_<A, B>(a: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn HasValue<bool, Output = bool>> {
    never_("NOT ".to_string() + &a.to_sql())
}

pub fn set_<'a, A, B>(lhs: Rc<dyn HasValue<A, Output = Column>>, rhs: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn 'a + HasSet>
where
    A: 'a + fmt::Display,
    B: 'a + ToLiteral,
{
    Rc::new(SetValue(lhs, rhs))
}

pub fn sum_<'a, A>(a: A) -> Rc<dyn 'a + HasValue<u32, Output = Column>>
where
    A: 'a + UnsafeSqlFunctionArgument,
{
    unsafe_sql_function("SUM", a, NeedParens::Parens)
}

pub fn count_<'a, A>(a: A) -> Rc<dyn 'a + HasValue<u32, Output = Column>>
where
    A: 'a + UnsafeSqlFunctionArgument,
{
    unsafe_sql_function("COUNT", a, NeedParens::Parens)
}

pub fn avg_<'a, A>(a: A) -> Rc<dyn 'a + HasValue<f32, Output = Column>>
where
    A: 'a + UnsafeSqlFunctionArgument,
{
    unsafe_sql_function("AVG", a, NeedParens::Parens)
}

pub fn round_<'a, A>(a: A) -> Rc<dyn 'a + HasValue<f32, Output = Column>>
where
    A: 'a + UnsafeSqlFunctionArgument,
{
    unsafe_sql_function("ROUND", a, NeedParens::Parens)
}

pub fn like_<'a, A, B>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: Rc<dyn HasValue<String, Output = String>>) -> Rc<dyn 'a + HasValue<bool, Output = String>>
where
    B: ToLiteral,
{
    let op: Rc<dyn HasValue<A, Output = B>> = never_(lhs.to_sql());
    parens_(op.to_sql() + " LIKE " + &rhs.to_sql())
}

pub fn ilike_<'a, A, B>(lhs: Rc<dyn HasValue<A, Output = B>>, rhs: Rc<dyn HasValue<String, Output = String>>) -> Rc<dyn 'a + HasValue<bool, Output = String>>
where
    B: ToLiteral,
{
    let op: Rc<dyn HasValue<A, Output = B>> = never_(lhs.to_sql());
    parens_(op.to_sql() + " ILIKE " + &rhs.to_sql())
}

pub fn don_<A, B>(a: Rc<dyn HasValue<A, Output = B>>) -> Box<dyn HasDistinct>
where
    A: 'static,
    B: 'static,
{
    Box::new(a)
}

pub fn case_<'a, A, B, C>(
    when: &[(Rc<dyn HasValue<bool, Output = bool>>, Rc<dyn HasValue<A, Output = B>>)],
    expr: Rc<dyn HasValue<A, Output = C>>,
) -> Rc<dyn 'a + HasValue<A, Output = C>>
where
    A: 'a + fmt::Display,
    B: 'a + ToLiteral,
    C: 'a + ToLiteral,
{
    let s = "CASE".to_string() + &map_when(when) + " ELSE " + &expr.to_sql() + " END";
    parens_(s)
}

fn map_when<A, B>(when: &[(Rc<dyn HasValue<bool, Output = bool>>, Rc<dyn HasValue<A, Output = B>>)]) -> String {
    when.iter()
        .fold(String::default(), |acc, (a, b)| acc + " WHEN " + &a.to_sql() + " THEN " + &b.to_sql())
}

pub fn when_<'a, A, B>(
    cond: Rc<dyn HasValue<bool, Output = bool>>,
    _: (),
    expr: Rc<dyn HasValue<A, Output = B>>,
) -> (Rc<dyn HasValue<bool, Output = bool>>, Rc<dyn HasValue<A, Output = B>>)
where
    B: 'a + ToLiteral,
{
    (cond, expr)
}

pub fn then_() -> () {
    ()
}

pub fn else_<A, B>(a: Rc<dyn HasValue<A, Output = B>>) -> Rc<dyn HasValue<A, Output = B>> {
    a
}

pub fn if_<'a, A, B, C>(
    cond: Rc<dyn HasValue<bool, Output = bool>>,
    expr: Rc<dyn HasValue<A, Output = B>>,
    _else: Rc<dyn HasValue<A, Output = C>>,
) -> Rc<dyn 'a + HasValue<A, Output = B>>
where
    A: 'a + fmt::Display,
    B: 'a + ToLiteral,
{
    let s = "IF(".to_string() + &cond.to_sql() + ", " + &expr.to_sql() + ", " + &_else.to_sql() + ")";
    parens_(s)
}

pub fn rank_() -> Rc<dyn HasValue<u32, Output=u32>> {
    never_("RANK()")
}

pub fn row_number_() -> Rc<dyn HasValue<u32, Output=u32>> {
    never_("ROW_NUMBER()")
}

pub fn lag_<'a, A, B, C>(
    column: Rc<dyn HasValue<A, Output = Column>>,
    offset: Option<Rc<dyn HasValue<u32, Output = u32>>>,
    default: Option<Rc<dyn HasValue<B, Output = C>>>
) -> Rc<dyn 'a + HasValue<A, Output=C>> where C: 'a + ToLiteral {
    lag_lead_("lag", column, offset, default)
}

pub fn lead_<'a, A, B, C>(
    column: Rc<dyn HasValue<A, Output = Column>>,
    offset: Option<Rc<dyn HasValue<u32, Output = u32>>>,
    default: Option<Rc<dyn HasValue<B, Output = C>>>
) -> Rc<dyn 'a + HasValue<A, Output=C>> where C: 'a + ToLiteral {
    lag_lead_("lead", column, offset, default)
}

fn lag_lead_<'a, A, B, C>(
    f: &str,
    column: Rc<dyn HasValue<A, Output = Column>>,
    offset: Option<Rc<dyn HasValue<u32, Output = u32>>>,
    default: Option<Rc<dyn HasValue<B, Output = C>>>
) -> Rc<dyn 'a + HasValue<A, Output=C>> where C: 'a + ToLiteral {
    let ret = match (column, offset, default) {
        (n, Some(a), Some(b)) => format!("{}({}, {}, {})", f, n.to_sql(), a.to_sql(), b.to_sql()),
        (n, Some(a), None) => format!("{}({}, {})", f, n.to_sql(), a.to_sql()),
        (n, _, _) => format!("{}({})", f, n.to_sql()),
        
    };
    never_(ret)
}

pub fn partition_by_<'a, A, B, C, D>(
    aggregate: Rc<dyn HasValue<A, Output = B>>,
    partition: Rc<dyn HasValue<C, Output = D>>,
    order: Option<Rc<dyn HasOrder>>,
) -> Rc<dyn 'a + HasValue<A, Output=B>> where A: 'a + fmt::Display, B: 'a + ToLiteral, D: 'a + ToLiteral {
    let s = aggregate.to_sql() + " OVER (" + &in_partition(partition, order) + ")";
    never_(s)
}

fn in_partition<A, B>(partition: Rc<dyn HasValue<A, Output = B>>, order: Option<Rc<dyn HasOrder>>) -> String {
    let mut s = "PARTITION BY ".to_string() + &partition.to_sql();
    if let Some(o) = order {
       s = s + " ORDER BY " + &o.to_string();
    }
    s
}