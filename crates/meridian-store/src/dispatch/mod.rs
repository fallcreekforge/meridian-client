mod steam;

use crate::{
   Platform,
   dispatch::steam::SteamDispatcher,
};

pub trait Dispatch: Send + Sync {
   // TODO: domain specific queries
}

#[expect(dead_code)]
pub struct CombinedDispatcher {
   collectors: Vec<Box<dyn Dispatch>>,
}

#[expect(dead_code)]
impl CombinedDispatcher {
   pub(crate) fn new(platforms: &[Platform]) -> Self {
      let collectors = platforms
         .iter()
         .map(|platform| -> Box<dyn Dispatch> {
            match *platform {
               Platform::Steam => Box::new(SteamDispatcher),
            }
         })
         .collect();

      Self { collectors }
   }
}

// TODO: enumerate dispatchers and aggregate results
impl Dispatch for CombinedDispatcher {}
