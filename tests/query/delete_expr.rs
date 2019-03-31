use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_delete() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        q.return_(a.user_id())
    });

    assert_eq!(delete(a.unwrap()).to_sql(), "DELETE FROM User WHERE (User.user_id = 1)".to_string());
}

#[test]
fn test_truncate() {
    let a = Query::<User>::from_();
    assert_eq!(truncate(a.unwrap()).to_sql(), "TRUNCATE TABLE User".to_string());
}
