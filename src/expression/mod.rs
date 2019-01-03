use std::rc::Rc;

use query::Query;
use types::*;

pub fn eq_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValue<L>>) -> Rc<HasValue<bool>> {
    let a = lhs.to_string();
    let b = rhs.to_string();

    Rc::new(Raw(NeedParens::Parens, a + " = " + &b))
}

pub fn not_eq_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValue<L>>) -> Rc<HasValue<bool>> {
    let a = lhs.to_string();
    let b = rhs.to_string();

    Rc::new(Raw(NeedParens::Parens, a + " <> " + &b))
}

pub fn in_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValueList<L>>) -> Rc<HasValue<bool>> {
    let comp = Raw(NeedParens::Parens, rhs.to_string());
    let op = binop_(" IN ", lhs, Rc::new(comp));

    if_not_empty_list(rhs, false, Rc::new(op))
}

pub fn not_in_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValueList<L>>) -> Rc<HasValue<bool>> {
    let comp = Raw(NeedParens::Parens, (*rhs).to_string());
    let op = binop_(" NOT IN ", lhs, Rc::new(comp));

    let res = if_not_empty_list(rhs, true, Rc::new(op));
    res
}

pub fn val_<A: ToString>(typ: A) -> Rc<HasValue<A>> {
    Rc::new(Raw(NeedParens::Never, typ.to_string()))
}

pub fn val_list_<A: 'static + ToString>(vs: &[Rc<HasValue<A>>]) -> Rc<HasValueList<A>> {
    if vs.is_empty() {
        return Rc::new(List::Empty);
    }

    let s = vs.to_vec().iter().map(|i| i.to_string().clone()).collect::<Vec<_>>().join(", ");

    let v = Raw(NeedParens::Parens, s.clone());
    let res = List::NonEmpty(Box::new(v));

    Rc::new(res)
}

fn if_not_empty_list<A>(v: Rc<HasValueList<A>>, b: bool, e: Rc<HasValue<bool>>) -> Rc<HasValue<bool>> {
    if (*v).is_empty() {
        return val_(b);
    };
    e
}

pub fn and_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValue<L>>) -> Rc<HasValue<L>> {
    Rc::new(binop_(" AND ", lhs, rhs))
}

pub fn or_<L>(lhs: Rc<HasValue<L>>, rhs: Rc<HasValue<L>>) -> Rc<HasValue<L>> {
    Rc::new(binop_(" OR ", lhs, rhs))
}

pub fn binop_<L>(op: &str, lhs: Rc<HasValue<L>>, rhs: Rc<HasValue<L>>) -> Raw {
    let a = (*lhs).to_string();
    let b = (*rhs).to_string();

    Raw(NeedParens::Parens, a + op + &b)
}

pub fn asc_<A: 'static + ToString>(exp: Rc<HasValue<A>>) -> Rc<HasOrder> {
    Rc::new(OrderBy(OrderByType::Asc, exp))
}

pub fn desc_<A: 'static + ToString>(exp: Rc<HasValue<A>>) -> Rc<HasOrder> {
    Rc::new(OrderBy(OrderByType::Desc, exp))
}

pub fn sub_<A: 'static + ToString>(q: Query<Rc<HasValue<A>>>) -> Rc<HasValue<A>> {
    let sql = q.to_sql();
    Rc::new(Raw(NeedParens::Parens, sql))
}
