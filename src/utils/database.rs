use std::env;
use std::pin::Pin;
use sqlx::{Connection, PgConnection};

pub(crate) async fn create_connection() -> PgConnection {
    let conn = PgConnection::connect(format!("postgres://{}:{}@{}",
                                  env::var("DATABASE_USERNAME")
                                      .unwrap(),
                                  env::var("DATABASE_PASSWORD")
                                      .unwrap(),
                                  env::var("DATABASE_URL")
                                      .unwrap()).as_str()).await;

    conn.unwrap()
}