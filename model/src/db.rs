// Essa lib vai configurar o modelo de dados da api REST
// podendo ser acessada por outros programas no localhost

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{Self, FromRequest};
use rocket::{Request, State, Outcome};

use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::pg::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
static DATABASE_URL: &'statix str = env!("DATABASE_URL");

pub fn connect() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'s Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard()::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &self::Target {
        &self.0
    }
}