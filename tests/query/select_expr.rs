use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_select() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        q.return_((a.user_id(), a.email(), val_(2)))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id, User.email, 2 FROM User WHERE (User.user_id = 1)".to_string()
    );
}

#[test]
fn test_distinct() {
    let a = Query::<User>::from_by(|q, a| {
        let q = q.distinct_on_(vec![don_(a.user_id()), don_(a.email())]);

        q.return_((a.user_id(), a.email()))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT DISTINCT ON (User.user_id, User.email) User.user_id, User.email FROM User".to_string()
    );
}

#[test]
fn test_functions() {
    let u = User::default();
    let sum = sum_(u.user_id());

    assert_eq!("SUM(User.user_id)", sum.to_string());

    let a = Query::<User>::from_by(|q, u| {
        let sum = sum_(u.user_id());
        let count = count_(u.user_id());
        let avg = avg_(u.user_id());

        q.return_((sum, count, avg))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT SUM(User.user_id), COUNT(User.user_id), AVG(User.user_id) FROM User".to_string()
    );
}
