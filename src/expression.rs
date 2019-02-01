use std::rc::Rc;
use std::fmt;

use crate::query::*;
use crate::types::*;


pub fn eq_<L, DB1, DB2>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Rc<HasValue<bool, bool>> {
    let a = lhs.to_sql();
    let b = rhs.to_sql();

    Rc::new(Raw(NeedParens::Parens, a + " = " + &b))
}

pub fn not_eq_<L, DB1, DB2>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Rc<HasValue<bool, bool>> {
    let a = lhs.to_string();
    let b = rhs.to_string();

    Rc::new(Raw(NeedParens::Parens, a + " <> " + &b))
}

pub fn in_<L: fmt::Display, DB1>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValueList<L>>) -> Rc<HasValue<bool, bool>> {
    let comp: Rc<HasValue<L, i32>> = Rc::new(Raw(NeedParens::Parens, rhs.to_string()));
    let op = binop_(" IN ", lhs, comp);

    if_not_empty_list(rhs, false, Rc::new(op))
}

pub fn not_in_<L: ToLiteral, DB1>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValueList<L>>) -> Rc<HasValue<bool, bool>> {
    let comp: Rc<HasValue<L, i32>> = Rc::new(Raw(NeedParens::Parens, rhs.to_string()));
    let op = binop_(" NOT IN ", lhs, comp);

    let res = if_not_empty_list(rhs, true, Rc::new(op));
    res
}

pub fn val_<A: fmt::Display + ToLiteral>(typ: A) -> Rc<HasValue<A, A>> {
    Rc::new(Raw(NeedParens::Never, typ.to_string()))
}

pub fn val_list_<'a, A, DB>(vs: &[Rc<HasValue<A, DB>>]) -> Rc<'a + HasValueList<A>> 
    where A: 'a + fmt::Display, DB: 'a + ToLiteral {
    if vs.is_empty() {
        let l: List<A, DB> = List::Empty;
        return Rc::new(l);
    }

    let s = vs.to_vec()
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>().join(", ");

    let v = Raw(NeedParens::Parens, s);

    Rc::new(List::NonEmpty(Box::new(v)) as List<A, DB>)
}

fn if_not_empty_list<A>(v: Rc<HasValueList<A>>, b: bool, e: Rc<HasValue<bool, bool>>) -> Rc<HasValue<bool, bool>> {
    match v {
        _ if v.is_empty() => val_(b),
        _ => e,
    }
}

pub fn and_<L, DB1: ToLiteral, DB2: ToLiteral>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Rc<HasValue<L, DB1>> {
    Rc::new(binop_(" AND ", lhs, rhs))
}

pub fn or_<L, DB1: ToLiteral, DB2: ToLiteral>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Rc<HasValue<L, DB1>> {
    Rc::new(binop_(" OR ", lhs, rhs))
}

pub fn binop_<L, DB1, DB2>(op: &str, lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Raw {
    let a = (*lhs).to_string();
    let b = (*rhs).to_string();

    Raw(NeedParens::Parens, a + op + &b)
}

pub fn between_<L, DB0: ToLiteral, DB1: ToLiteral, DB2: ToLiteral>(comp: Rc<HasValue<L, DB0>>, lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValue<L, DB2>>) -> Rc<HasValue<bool, bool>> {
    let e = (*comp).to_string();
    let a = (*lhs).to_string();
    let b = (*rhs).to_string();

    Rc::new(Raw(NeedParens::Parens, e + " BETWEEN " + &a + " TO " + &b))
}

pub fn is_null_<A, DB>(a: Rc<HasValue<A, DB>>) -> Rc<HasValue<bool, bool>> 
    where A: fmt::Display, DB: ToLiteral {
    Rc::new(Raw(NeedParens::Parens, a.to_string() + " IS NULL"))
}

pub fn is_not_null_<A, DB>(a: Rc<HasValue<A, DB>>) -> Rc<HasValue<bool, bool>> 
    where A: fmt::Display, DB: ToLiteral {
    Rc::new(Raw(NeedParens::Parens, a.to_string() + " IS NOT NULL"))
}

pub fn asc_<'a, A, DB>(exp: Rc<HasValue<A, DB>>) -> Rc<'a + HasOrder> 
    where A: 'a + fmt::Display, DB: 'a + ToLiteral {
    Rc::new(OrderBy(OrderByType::Asc, exp))
}

pub fn desc_<'a, A, DB>(exp: Rc<HasValue<A, DB>>) -> Rc<'a + HasOrder> 
    where A: 'a + fmt::Display, DB: 'a + ToLiteral {
    Rc::new(OrderBy(OrderByType::Desc, exp))
}

pub fn sub_<'a, A, DB>(q: Query<Rc<HasValue<A, DB>>>) -> Rc<'a + HasValue<A, DB>> 
    where A: 'a + fmt::Display, DB: 'a + ToLiteral {
    Rc::new(Raw(NeedParens::Parens, select(q).to_sql()))
}

fn unsafe_sql_function<A, B, DB>(name: &str, arg: A) -> Rc<HasValue<B, DB>> 
    where A: UnsafeSqlFunctionArgument, DB: ToLiteral {
    let args = A::to_arg_list(arg);
    let results = args.iter().map(|v| v.to_string()).collect::<Vec<_>>();

    Rc::new(Raw(NeedParens::Never, format!("{}({})", name, results.join(","))))
}

pub fn set_<'a, L, DB1>(lhs: Rc<HasValue<L, Column>>, rhs: Rc<HasValue<L, DB1>>) -> Rc<'a + HasSet> 
    where L: 'a + fmt::Display, DB1: 'a + ToLiteral {
    Rc::new(SetValue(lhs, rhs))
}

use crate::entity::*;

pub fn sum_<'a, A>(a: A) -> Rc<'a + HasValue<u32, Column>> 
    where A: 'a + UnsafeSqlFunctionArgument {
    unsafe_sql_function("SUM", a)
}

pub fn count_<'a, A>(a: A) -> Rc<'a + HasValue<u32, Column>> 
    where A: 'a + UnsafeSqlFunctionArgument {
    unsafe_sql_function("COUNT", a)
}

pub fn avg_<'a, A>(a: A) -> Rc<'a + HasValue<f32, Column>> 
    where A: 'a + UnsafeSqlFunctionArgument {
    unsafe_sql_function("AVG", a)
}