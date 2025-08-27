use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use crate::models::Authentication;
use crate::schema::authentication::dsl;
use crate::AuthRepo;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DieselRepo {
    pool: PgPool,
}

impl DieselRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthRepo for DieselRepo {
    async fn find_user(&self, username: &str) -> Result<Option<Authentication>, anyhow::Error> {
        let username = username.to_string();
        let pool = self.pool.clone();

        let row: Option<Authentication> = web::block(move || -> Result<Option<Authentication>, anyhow::Error> {
                let mut conn = pool.get()?; // r2d2::Error -> anyhow
                let row = dsl::authentication
                    .filter(dsl::username.eq(username))
                    .select(Authentication::as_select())
                    .first::<Authentication>(&mut conn)
                    .optional()?; // diesel::Error -> anyhow
                Ok(row)
            })
            .await
            .map_err(|e| anyhow::anyhow!(e))??;

        Ok(row)
    }
}

