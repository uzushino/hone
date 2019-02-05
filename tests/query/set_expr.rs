use hone::expression::*;
use hone::query::*;
use hone::types::*;

use crate::query::model::*;

#[test]
fn test_set() {
    let a = Query::<User>::from_by(|q, a| {
        let email1 = val_("a@b.c".to_string());
        let eq = eq_(a.email(), email1);
        let q = q.where_(eq);
        
        let email2 = val_("d@e.f".to_string());
        let one = val_(1);
        
        let set1 = set_(a.user_id(), one);
        let set2 = set_(a.email(), email2);

        let q = q.value_(set1);
        let q = q.value_(set2);

        q
    });

    assert_eq!(
        update(a.unwrap()).to_sql(),
        "UPDATE User SET User.user_id = 1, User.email = 'd@e.f' \
        WHERE (User.email = 'a@b.c')"
            .to_string()
    );
}

#[test]
fn test_set_join() {
    let a = Query::<InnerJoin<_, _>>::from_by(|q, InnerJoin(a, b): InnerJoin<User, Library>| {
        let email1 = val_("a@b.c".to_string());
        let eq = eq_(a.email(), email1);
        let q = q.where_(eq);
        
        let on_eq = eq_(a.user_id(), b.library_id());
        let q = q.on_(on_eq);
        
        let email2 = val_("d@e.f".to_string());
        let one = val_(1);
        
        let set1 = set_(a.user_id(), one);
        let set2 = set_(a.email(), email2);

        let q = q.value_(set1);
        let q = q.value_(set2);

        q.return_(a)
    });

    assert_eq!(
        update(a.unwrap()).to_sql(),
        "UPDATE User INNER JOIN Library ON (User.user_id = Library.library_id) SET User.user_id = 1, User.email = 'd@e.f' \
        WHERE (User.email = 'a@b.c')"
            .to_string()
    );

}

#[test]
fn test_update_select() {
    let u = Query::<User>::from_by(|q, u| {
        let one = val_(1);
        let eq = eq_(u.user_id(), one);
        let q = q.where_(eq);

        q.return_((u.user_id(), u.email()))
    });
    let u = u.unwrap();

    let q = update_select(u, |q: Query<Library>, l, u| {
        let set1 = set_(l.library_id(), u.value.0.clone());
        let set2 = set_(l.title(), u.value.1.clone());

        let q = q.value_(set1);
        let q = q.value_(set2);
        
        q
    });

    assert_eq!(
        q.to_sql(),
        "UPDATE Library \
         SET Library.library_id = User.user_id, Library.title = User.email \
         FROM User WHERE (User.user_id = 1)".to_string()
    );
}