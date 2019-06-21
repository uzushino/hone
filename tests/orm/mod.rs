use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use diesel::prelude::*;
use diesel::sql_query;
use diesel_migrations;

use hone::entity::{Column, HasEntityDef, Table as Tbl};
use hone::expression::*;
use hone::query::*;
use hone::types::*;

fn establish_connection() -> SqliteConnection {
    let database_url = "/tmp/hoge.db";

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

table! {
    downloads {
        id -> Integer,
        version -> Text,
    }
}

#[derive(Debug, Queryable, QueryableByName, Insertable, Default)]
#[table_name = "downloads"]
struct Download {
    pub id: i32,
    pub version: String,
}

impl Download {
    pub fn id(&self) -> Rc<HasValue<i32, Output=Column>> {
        never_("downloads.id")
    }

    pub fn version(&self) -> Rc<HasValue<String, Output=Column>> {
        never_("downloads.version")
    }
}

impl HasEntityDef for Download {
    fn table_name() -> Tbl {
        Tbl::new("downloads", None)
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "id",
            "version"
        ]
    }
}

impl HasQuery for Download {
    type T = Download;
}

fn setup() {
    let connection = establish_connection();
    let path = Path::new("tests/migrations");
    let mut output = File::create("/tmp/output.txt").unwrap();

    let _ = diesel_migrations::run_pending_migrations_in_directory(&connection, path, &mut output);
}

#[test]
fn test_diesel() {
    use super::orm::downloads::dsl::*;

    setup();

    let connection = establish_connection();

    let a = downloads.filter(id.eq(1)).load::<Download>(&connection).unwrap();
    let a = a.first().unwrap();
    let b = Query::<Download>::from_by(|q, m| {
        let id_ = val_(1);
        let eq = eq_(m.id(), id_);
        let q = q.where_(eq);
        q
    });

    let b = sql_query(select(b.unwrap()).to_sql()).load::<Download>(&connection).unwrap();

    let b = b.first().unwrap();

    assert_eq!(a.version, b.version);
}
