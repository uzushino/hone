use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_insert_into() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let email1 = val_("a@b.c".to_string());

        let q = q.value_(a.user_id(), one);
        let q = q.value_(a.email(), email1);

        q
    });

    assert_eq!(
        insert_into(a.unwrap()).to_sql(),
        "INSERT INTO User(User.user_id, User.email) VALUES (1, 'a@b.c')".to_string()
    );
}

#[test]
fn test_insert_select() {
    let u = Query::<User>::from_by(|q, u| {
        let one = val_(1);
        let eq = eq_(u.user_id(), one);
        let q = q.where_(eq);

        q.return_((u.user_id(), u.email()))
    });
    let u = u.unwrap();

    let q = insert_select(u, |q: Query<Library>, l, u| {
        let q = q.value_(l.library_id(), u.value.0.clone());
        let q = q.value_(l.title(), u.value.1.clone());
        
        q
    });

    assert_eq!(
        q.to_sql(),
        "INSERT INTO Library(Library.library_id, Library.title) \
         SELECT User.user_id, User.email FROM User WHERE (User.user_id = 1)".to_string()
    );
}