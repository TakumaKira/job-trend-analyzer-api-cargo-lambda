use diesel::prelude::*;
use crate::db::models::Result;

pub fn get_result(conn: &mut PgConnection, target_url: String) -> Vec<Result> {
    use crate::db::schema::results::dsl::*;
    results.filter(url.eq(target_url)).select(Result::as_select()).load(conn).expect("Error loading results")
}
