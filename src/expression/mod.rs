use std::rc::Rc;

use query::Query;
use types::*;

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

pub fn in_<L: ToString, DB1>(lhs: Rc<HasValue<L, DB1>>, rhs: Rc<HasValueList<L>>) -> Rc<HasValue<bool, bool>> {
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

pub fn val_<A: ToString + ToLiteral>(typ: A) -> Rc<HasValue<A, A>> {
    Rc::new(Raw(NeedParens::Never, typ.to_string()))
}

pub fn val_list_<A: 'static + ToString, DB: 'static + ToLiteral>(vs: &[Rc<HasValue<A, DB>>]) -> Rc<HasValueList<A>> {
    if vs.is_empty() {
        let l: List<A, DB> = List::Empty;
        return Rc::new(l);
    }

    let s = vs.to_vec().iter().map(|i| i.to_string().clone()).collect::<Vec<_>>().join(", ");

    let v = Raw(NeedParens::Parens, s.clone());
    let res: List<A, DB> = List::NonEmpty(Box::new(v));

    Rc::new(res)
}

fn if_not_empty_list<A>(v: Rc<HasValueList<A>>, b: bool, e: Rc<HasValue<bool, bool>>) -> Rc<HasValue<bool, bool>> {
    if (*v).is_empty() {
        return val_(b);
    };
    e
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

pub fn asc_<A: 'static + ToString, DB: 'static + ToLiteral>(exp: Rc<HasValue<A, DB>>) -> Rc<HasOrder> {
    Rc::new(OrderBy(OrderByType::Asc, exp))
}

pub fn desc_<A: 'static + ToString, DB: 'static + ToLiteral>(exp: Rc<HasValue<A, DB>>) -> Rc<HasOrder> {
    Rc::new(OrderBy(OrderByType::Desc, exp))
}

pub fn sub_<A: 'static + ToString, DB: ToLiteral>(q: Query<Rc<HasValue<A, DB>>>) -> Rc<HasValue<A, DB>> {
    let sql = q.to_sql();
    Rc::new(Raw(NeedParens::Parens, sql))
}