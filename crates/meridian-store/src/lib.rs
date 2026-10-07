//! SQLx-backed SQLite persistence for Meridian Client.
use sqlx::SqlitePool;

#[expect(dead_code)]
#[derive(Debug)]
pub struct DbConnection {
   path: String,
   pool: Option<SqlitePool>,
}

impl DbConnection {
   pub fn new(path: impl AsRef<str>) -> Self {
      Self {
         path: path.as_ref().to_owned(),
         pool: None,
      }
   }

   // fn get_inner(&self) -> Result<&SqlitePool> {
   // self.pool.as_ref().ok_or_else(|| )
   // }
}
