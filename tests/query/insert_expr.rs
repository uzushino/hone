use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_insert_into() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let email1 = val_("a@b.c".to_string());
        
        let set1 = set_(a.user_id(), one);
        let set2 = set_(a.email(), email1);

        let q = q.value_(set1);
        let q = q.value_(set2);

        q
    });

    assert_eq!(
        insert_into(a.unwrap()).to_sql(),
        "INSERT INTO User(User.user_id, User.email) VALUES (1, 'a@b.c')".to_string()
    );
}