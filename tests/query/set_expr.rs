use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_set() {
    let a = Query::<User>::from_by(|q, a| {
        let email1 = val_("a@b.c".to_string());
        let eq = eq_(a.email(), email1);
        let q = q.where_(eq);
        
        let email2 = val_("d@e.f".to_string());
        let one = val_(1);

        let q = q.value_(a.user_id(), one);
        let q = q.value_(a.email(), email2);

        q
    });

    assert_eq!(
        update(a.unwrap()).to_sql(),
        "UPDATE User SET User.user_id = 1, User.email = 'd@e.f' \
        WHERE (User.email = 'a@b.c')"
            .to_string()
    );
}