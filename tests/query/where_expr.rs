use hone::expression::*;
use hone::query::*;
use hone::types::*;

use query::model::*;

use std::rc::Rc;

#[test]
fn test_eq() {
    let u = User::default();
    let zero: Rc<HasValue<String>> = val_("aaaa".to_string());
    let eq = eq_(u.email(), zero);

    assert_eq!("(User.user_id = 1)", eq.to_string());
}

#[test]
fn test_not_eq() {
    let u = User::default();
    let one = val_(1);
    let neq = not_eq_(u.user_id(), one);

    assert_eq!("(User.user_id <> 1)", neq.to_string());
}

#[test]
fn test_where() {
    let a = Query::<User>::from_by(|q, a| {
        let one = val_(1);
        let eq = eq_(a.user_id(), one);
        let q = q.where_(eq);

        q.return_(a.user_id())
    });

    assert_eq!(a.unwrap().to_sql(), "SELECT User.user_id FROM User WHERE (User.user_id = 1)".to_string());

    let b = Query::<(_, _)>::from_by(|q, (a, b): (User, Library)| {
        let eq = eq_(a.user_id(), b.library_id());
        q.where_(eq)
    });

    assert_eq!(
        b.unwrap().to_sql(),
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
        c.unwrap().to_sql(),
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
        d.unwrap().to_sql(),
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
        e.unwrap().to_sql(),
        "SELECT email, user_id FROM User WHERE (User.user_id IN ((1, 2, 3)))".to_string()
    );

    let f = Query::<(User, Library)>::from_by(|q, (a, b)| {
        let one = val_(1);
        let two = val_(2);
        let three = val_(3);
        let eq = in_(a.user_id(), val_list_(&[one, two, three, b.library_id()]));
        let q = q.where_(eq);

        q.return_(a)
    });

    assert_eq!(
        f.unwrap().to_sql(),
        "SELECT email, user_id FROM Library,User WHERE (User.user_id IN ((1, 2, 3, Library.library_id)))".to_string()
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
        g.unwrap().to_sql(),
        "SELECT email, user_id FROM User WHERE ((User.user_id = 1) AND (User.email = (SELECT Library.title FROM Library WHERE \
         (Library.library_id = 2))))"
            .to_string()
    );
}
