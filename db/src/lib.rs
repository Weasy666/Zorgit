use owners::Owners;
use projects::Projects;
use rocket::{Request, try_outcome, State, request::{self, FromRequest}};
use sqlx::{Pool, Postgres, postgres::PgPool};
use users::Users;

mod owners;
mod projects;
mod users;


pub struct Database {
    pool: PgPool,
    pub users: Users,
    pub owners: Owners,
    pub projects: Projects,
}

impl Database {
    fn with_pool(pool: Pool<Postgres>) -> Database {
        //INFO: This should be ok, because sqlx::Pool internally holds an Arc<T> of the SharedPool
        Database {
            pool: pool.clone(),
            users: Users::with_pool(pool.clone()),
            owners: Owners::with_pool(pool.clone()),
            projects: Projects::with_pool(pool),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Database {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = try_outcome!(request.guard::<State<Database>>().await);
        //INFO: This should be ok, because sqlx::Pool internally holds an Arc<T> of the SharedPool
        request::Outcome::Success(Database::with_pool(db.pool.clone()))
    }
}
