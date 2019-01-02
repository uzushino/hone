use hone::expression::*;
use hone::query::*;

use query::model::*;

#[test]
fn test_order() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);

        let q = q.where_(eq);
        let q = q.order_(vec![asc_(a.user_id()), desc_(a.email())]);

        q.return_(a)
    });

    assert_eq!(
        a.unwrap().to_sql(),
        "SELECT email, user_id FROM User WHERE (User.user_id = 1) \
         ORDER BY User.user_id ASC, User.email DESC"
            .to_string()
    );
}
