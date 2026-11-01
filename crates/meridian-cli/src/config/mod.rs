mod paths;
mod schema;

pub use paths::default_config_path;
pub use schema::Config;

#[cfg(test)] mod tests;
