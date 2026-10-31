mod json_file;

pub use json_file::{
   JsonFileError,
   load_json_file,
};

#[cfg(test)] mod tests;
