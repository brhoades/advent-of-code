pub mod coord;
#[cfg(test)]
mod logging;
pub mod map;
pub mod neighbor_map;
pub mod year_2022;
pub mod year_2023;

pub mod prelude {
    #[cfg(test)]
    pub use crate::logging::init as init_logging;
    pub use anyhow::{anyhow, bail, ensure, Context, Error, Result};
    pub use derive_deref::{Deref, DerefMut};
    pub use log::{debug, error, info, trace, warn};
    pub use std::str::FromStr;
    pub use thiserror::Error;
}
