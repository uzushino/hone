use hone::expression::*;
use hone::query::*;
use hone::types::*;

use crate::query::model::*;

#[test]
fn test_inner_join() {
    let a = Query::<InnerJoin<_, _>>::from_by(|q, InnerJoin(a, b): InnerJoin<User, Library>| {
        let one = val_(1);

        let on_eq = eq_(a.user_id(), b.library_id());
        let eq = eq_(a.user_id(), one);

        let q = q.on_(on_eq);
        let q = q.where_(eq);

        q.return_((a, b))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT email, user_id, library_id, title FROM \
         User INNER JOIN Library ON (User.user_id = Library.library_id) \
         WHERE (User.user_id = 1)"
            .to_string()
    );
}

#[test]
fn test_left_join() {
    let b = Query::<LeftJoin<_, _>>::from_by(|q, LeftJoin(a, InnerJoin(b, c)): LeftJoin<User, InnerJoin<Library, Library>>| {
        let one = val_(1);

        let on_eq1 = eq_(a.user_id(), b.library_id());
        let on_eq2 = eq_(b.library_id(), c.library_id());

        let eq = eq_(a.user_id(), one);

        let q = q.on_(on_eq1);
        let q = q.on_(on_eq2);
        let q = q.where_(eq);

        q.return_(a)
    });

    assert_eq!(
        select(b.unwrap()).to_sql(),
        "SELECT email, user_id FROM \
         User LEFT OUTER JOIN Library INNER JOIN Library ON (User.user_id = Library.library_id) ON \
         (Library.library_id = Library.library_id) \
         WHERE (User.user_id = 1)"
            .to_string()
    );
}

#[test]
fn test_right_join() {
    let a = Query::<RightJoin<_, _>>::from_by(|q, RightJoin(a, b): RightJoin<User, Library>| {
        let one = val_(1);

        let on_eq = eq_(a.user_id(), b.library_id());
        let eq = eq_(a.user_id(), one);

        let q = q.on_(on_eq);
        let q = q.where_(eq);

        q.return_((a, b))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT email, user_id, library_id, title FROM \
         User RIGHT OUTER JOIN Library ON (User.user_id = Library.library_id) \
         WHERE (User.user_id = 1)"
            .to_string()
    );
}

#[test]
fn test_nested_inner_join() {
    let c = Query::<InnerJoin<_, _>>::from_by(
        |q, InnerJoin(InnerJoin(InnerJoin(a, b), _), _): InnerJoin<InnerJoin<InnerJoin<User, Library>, Library>, Library>| {
            let one = val_(1);

            let on_eq1 = eq_(a.user_id(), b.library_id());
            let on_eq2 = eq_(a.user_id(), b.library_id());
            let on_eq3 = eq_(a.user_id(), b.library_id());
            let eq = eq_(a.user_id(), one);

            let q = q.on_(on_eq1);
            let q = q.on_(on_eq2);
            let q = q.on_(on_eq3);
            let q = q.where_(eq);

            q.return_(a)
        },
    );

    assert_eq!(
        select(c.unwrap()).to_sql(),
        "SELECT email, user_id FROM User \
         INNER JOIN Library ON (User.user_id = Library.library_id) \
         INNER JOIN Library ON (User.user_id = Library.library_id) \
         INNER JOIN Library ON (User.user_id = Library.library_id) WHERE (User.user_id = 1)"
            .to_string()
    );
}
