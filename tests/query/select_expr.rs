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
fn test_star() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        q.return_(star_::<User>())
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.* FROM User WHERE (User.user_id = 1)".to_string()
    );
}

#[test]
fn test_alias_column() {
    let a = Query::<User>::from_by(|q, a| {
        let u = a.user_id();
        let one = val_("a@b.c".to_string());
        q.return_((u.as_("uid".to_string()), one.as_("email".to_string())))
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id AS uid, 'a@b.c' AS email FROM User".to_string()
    );

    let b = Query::<User>::from_by(|q, a| {
        let sub = Query::<User>::from_by(|q, u| {
            let one = val_(1);
            let eq = eq_(a.user_id(), one);
            let q = q.where_(eq);
            q.return_(u.user_id())
        })
        .unwrap();

        q.return_(sub_(sub).as_("user_id".to_string()))
    });

    assert_eq!(
        select(b.unwrap()).to_sql(),
        "SELECT (SELECT User.user_id FROM User WHERE (User.user_id = 1)) AS user_id FROM User".to_string()
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

#[test]
fn test_exists() {
    let a = Query::<User>::from_by(|q, _| {
        let sub = Query::<User>::from_by(|q, u| q.return_(u.user_id())).unwrap();
        let q = q.where_(exists_(sub));

        q
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT email, user_id FROM User WHERE EXISTS (SELECT User.user_id FROM User)".to_string()
    );

    let a = Query::<User>::from_by(|q, _| {
        let sub = Query::<User>::from_by(|q, u| q.return_(u.user_id())).unwrap();
        let q = q.where_(not_(exists_(sub)));

        q
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT email, user_id FROM User WHERE NOT EXISTS (SELECT User.user_id FROM User)".to_string()
    );
}

#[test]
fn test_case() {
    let a = Query::<User>::from_by(|q, a| {
        let sub1 = Query::<User>::from_by(|q, u| {
            let one = val_(1);
            let eq = eq_(a.user_id(), one);
            let q = q.where_(eq);
            q.return_(u.email())
        })
        .unwrap();

        let sub2 = Query::<User>::from_by(|q, u| {
            let one = val_(1);
            let eq = eq_(a.user_id(), one);
            let q = q.where_(eq);
            let q = q.limit_(1);

            q.return_(u.user_id())
        })
        .unwrap();

        let one = val_(1u32);
        let case = case_(&[when_(exists_(sub1), then_(), sub_(sub2))], one);

        q.return_(case)
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT (CASE \
         WHEN EXISTS (SELECT User.email FROM User WHERE (User.user_id = 1)) \
         THEN (SELECT User.user_id FROM User WHERE (User.user_id = 1) LIMIT 1) \
         ELSE 1 \
         END) FROM User"
            .to_string()
    );
}
