use rocket::http::{Status};
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;

use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool,PooledConnection, PoolError};

pub mod models;

pub use self::models::schema;

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

lazy_static! {
	pub static ref DB_POOL: SqlitePool = init_pool();
}

pub fn init_pool() -> SqlitePool {
	let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
	Pool::new(manager).expect("db pool")
}


pub struct DB(PooledConnection<ConnectionManager<SqliteConnection>>);

impl DB {
	pub fn conn(&self) -> &SqliteConnection {
		&*self.0
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
	type Error = PoolError;
	fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match DB_POOL.get() {
			Ok(conn) => Outcome::Success(DB(conn)),
			Err(e) => Outcome::Failure((Status::InternalServerError, e))
		}
	}
}
