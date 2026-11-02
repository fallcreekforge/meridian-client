mod steam;

use crate::{
   Platform,
   dispatch::steam::SteamDispatcher,
};

pub trait Dispatch: Send + Sync {
   // TODO: domain specific queries
}

pub struct CombinedDispatcher {
   #[expect(dead_code)]
   collectors: Vec<Box<dyn Dispatch>>,
}

impl CombinedDispatcher {
   #[expect(dead_code)]
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
