use sqlx::{PgPool, Pool, Postgres};


pub struct Users {
    pool: PgPool
}

impl Users {
    pub(crate) fn with_pool(pool: Pool<Postgres>) -> Users {
        Users {
            pool,
        }
    }
}
