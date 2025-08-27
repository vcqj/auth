use crate::schema::authentication;
use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = authentication)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Authentication {
    pub username: String,
    pub created: DateTime<Utc>,
    pub privilege: String,
    pub password: String,  // PHC Argon2 hash
}

