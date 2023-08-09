pub mod ministry_event;
pub mod person;

type TConn = sqlx::pool::Pool<sqlx::Sqlite>;
