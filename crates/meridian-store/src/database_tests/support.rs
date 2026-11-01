use std::{
   env,
   fs,
   path::PathBuf,
   process,
   sync::atomic::{
      AtomicU64,
      Ordering,
   },
};

use sqlx::{
   Connection as _,
   SqliteConnection,
   migrate::Migrator,
   sqlite::SqliteConnectOptions,
};

use crate::Outbox;

static NEXT_DATABASE: AtomicU64 = AtomicU64::new(0);

pub(super) struct TestDatabase {
   pub(super) directory: PathBuf,
   pub(super) path:      PathBuf,
}

impl TestDatabase {
   #[expect(
      clippy::create_dir,
      reason = "Fail if the isolated test directory already exists."
   )]
   pub(super) fn new() -> Self {
      let directory = env::temp_dir().join(format!(
         "meridian-database-{}-{}",
         process::id(),
         NEXT_DATABASE.fetch_add(1, Ordering::Relaxed)
      ));
      fs::create_dir(&directory).unwrap();
      let path = directory.join("state %23.db");
      fs::File::options()
         .write(true)
         .create_new(true)
         .open(&path)
         .unwrap();
      Self { directory, path }
   }

   pub(super) async fn open(&self) -> Outbox {
      let outbox = Outbox::open(&self.path)
         .await
         .expect("open the test database");
      sqlx::raw_sql(include_str!("../../tests/fixtures/database.sql"))
         .execute(&outbox.pool)
         .await
         .unwrap();
      outbox
   }

   pub(super) async fn connection(&self) -> SqliteConnection {
      SqliteConnection::connect_with(&SqliteConnectOptions::new().filename(&self.path))
         .await
         .unwrap()
   }

   pub(super) async fn migrator(&self, scripts: &[(&str, &str)]) -> Migrator {
      let directory = self.directory.join("migrations");
      fs::create_dir_all(&directory).unwrap();
      fs::write(
         directory.join("atlas.sum"),
         "synthetic metadata ignored by SQLx\n",
      )
      .unwrap();
      for &(name, sql) in scripts {
         fs::write(directory.join(name), sql).unwrap();
      }
      Migrator::new(directory).await.unwrap()
   }
}

impl Drop for TestDatabase {
   fn drop(&mut self) {
      let _ = fs::remove_dir_all(&self.directory);
   }
}
