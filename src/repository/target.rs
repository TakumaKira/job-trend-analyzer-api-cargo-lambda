use diesel::prelude::*;
use crate::db::models::Target;

pub fn get_targets(conn: &mut PgConnection) -> Vec<Target> {
    use crate::db::schema::targets::dsl::*;
    targets.select(Target::as_select()).load(conn).expect("Error loading targets")
}
