use sqlx::{PgPool, Pool, Postgres};


pub struct Projects {
    pool: PgPool
}

impl Projects {
    pub(crate) fn with_pool(pool: Pool<Postgres>) -> Projects {
        Projects {
            pool,
        }
    }
}
