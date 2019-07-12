use hone::expression::*;
use hone::query::*;

use crate::query::model::*;

#[test]
fn test_eq() {
    let u = User::default();
    let one = val_("a@b.c".to_string());
    let eq = eq_(u.email(), one);

    assert_eq!("(User.email = 'a@b.c')", eq.to_string());
}

#[test]
fn test_not_eq() {
    let u = User::default();
    let one = val_(1);
    let neq = not_eq_(u.user_id(), one);

    assert_eq!("(User.user_id <> 1)", neq.to_string());
}

#[test]
fn test_like() {
    let u = User::default();
    let one = val_("%aaa%".to_string());
    let eq = like_(u.user_id(), one);

    assert_eq!("(User.user_id LIKE '%aaa%')", eq.to_string());
}

#[test]
fn test_between() {
    let u = User::default();
    let one = val_(1);
    let two = val_(2);
    let between = between_(u.user_id(), one, two);

    assert_eq!("(User.user_id BETWEEN 1 TO 2)", between.to_string());
}

#[test]
fn test_relational_operator() {
    let u = User::default();
    let one = val_(1);

    let q = gt_(u.user_id(), one);
    assert_eq!("(User.user_id > 1)", q.to_string());

    let one = val_(1);
    let q = gte_(u.user_id(), one);
    assert_eq!("(User.user_id >= 1)", q.to_string());

    let one = val_(1);
    let q = lt_(u.user_id(), one);
    assert_eq!("(User.user_id < 1)", q.to_string());

    let one = val_(1);
    let q = lte_(u.user_id(), one);
    assert_eq!("(User.user_id <= 1)", q.to_string());
}

#[test]
fn test_is_null() {
    let u = User::default();
    let is_null = is_null_(u.user_id());
    let is_not_null = is_not_null_(u.user_id());
    let between = and_(is_null, is_not_null);

    assert_eq!("((User.user_id IS NULL) AND (User.user_id IS NOT NULL))", between.to_string());
}

#[test]
fn test_limit_offset() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);
        let q = q.limit_(100);
        let q = q.offset_(200);

        q.return_(a.user_id())
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id FROM User WHERE (User.user_id = 1) LIMIT 100 OFFSET 200".to_string()
    );
}

#[test]
fn test_groupby() {
    let a = Query::<User>::from_by(|q, a| {
        let q = q.group_by_(a.user_id());
        let q = q.group_by_(a.email());
        q.return_(a.user_id())
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id FROM User GROUP BY User.user_id, User.email".to_string()
    );
}

#[test]
fn test_having() {
    let a = Query::<User>::from_by(|q, a| {
        let q = q.group_by_(a.user_id());
        let q = q.group_by_(a.email());

        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.having_(eq);

        q.return_(a.user_id())
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id FROM User GROUP BY User.user_id, User.email \
         HAVING (User.user_id = 1)"
            .to_string()
    );
}

#[test]
fn test_where() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        q.return_(a.user_id())
    });

    assert_eq!(
        select(a.unwrap()).to_sql(),
        "SELECT User.user_id FROM User WHERE (User.user_id = 1)".to_string()
    );

    let b = Query::<(_, _)>::from_by(|q, (a, b): (User, Library)| {
        let eq = eq_(a.user_id(), b.library_id());
        q.where_(eq)
    });

    assert_eq!(
        select(b.unwrap()).to_sql(),
        "SELECT email, user_id, library_id, title FROM Library,User \
         WHERE (User.user_id = Library.library_id)"
            .to_string()
    );

    let c = Query::<(_, _)>::from_by(|q, (a, b): (User, Library)| {
        let one = val_(1);
        let two = val_(2);

        let eq1 = eq_(a.user_id(), one);
        let eq2 = eq_(b.library_id(), two);

        let q = q.where_(eq1);
        let q = q.where_(eq2);

        q
    });

    assert_eq!(
        select(c.unwrap()).to_sql(),
        "SELECT email, user_id, library_id, title FROM Library,User \
         WHERE ((User.user_id = 1) AND (Library.library_id = 2))"
            .to_string()
    );

    let d = Query::<(_, _)>::from_by(|q, (a, b): (User, Library)| {
        let one = val_(1);
        let two = val_(2);

        let eq1 = eq_(a.user_id(), one);
        let eq2 = eq_(b.library_id(), two);

        let q = q.where_(or_(eq1, eq2));

        q
    });

    assert_eq!(
        select(d.unwrap()).to_sql(),
        "SELECT email, user_id, library_id, title FROM Library,User \
         WHERE ((User.user_id = 1) OR (Library.library_id = 2))"
            .to_string()
    );

    let e = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let two = val_(2);
        let three = val_(3);
        let eq = in_(a.user_id(), val_list_(&[one, two, three]));

        q.where_(eq).return_(a)
    });

    assert_eq!(
        select(e.unwrap()).to_sql(),
        "SELECT email, user_id FROM User WHERE (User.user_id IN ((1, 2, 3)))".to_string()
    );

    let f = Query::<(User, Library)>::from_by(|q, (a, _b)| {
        let one = val_(1);
        let two = val_(2);
        let three = val_(3);
        let eq = in_(a.user_id(), val_list_(&[one, two, three]));
        let q = q.where_(eq);

        q.return_(a)
    });

    assert_eq!(
        select(f.unwrap()).to_sql(),
        "SELECT email, user_id FROM Library,User WHERE (User.user_id IN ((1, 2, 3)))".to_string()
    );

    let g = Query::<User>::from_by(|q, a| {
        let one = val_(1);

        let q2 = Query::<Library>::from_by(|q, b| {
            let two = val_(2);

            let eq = eq_(b.library_id(), two);
            let q = q.where_(eq);

            q.return_(b.title())
        });

        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        let sql_sub = q2.unwrap();
        let eq_sub = eq_(a.email(), sub_(sql_sub));
        let q = q.where_(eq_sub);

        q.return_(a)
    });

    assert_eq!(
        select(g.unwrap()).to_sql(),
        "SELECT email, user_id FROM User WHERE ((User.user_id = 1) AND (User.email = (SELECT Library.title FROM Library WHERE \
         (Library.library_id = 2))))"
            .to_string()
    );
}
