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

    assert_eq!(select(a.unwrap()).to_sql(), "SELECT User.user_id, User.email, 2 FROM User WHERE (User.user_id = 1)".to_string());
}