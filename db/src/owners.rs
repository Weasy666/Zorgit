use sqlx::{PgPool, Pool, Postgres};


pub struct Owners {
    pool: PgPool
}

impl Owners {
    pub(crate) fn with_pool(pool: Pool<Postgres>) -> Owners {
        Owners {
            pool,
        }
    }
}
