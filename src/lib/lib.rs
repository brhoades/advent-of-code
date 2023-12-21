pub mod coord;
pub mod map;
pub mod neighbor_map;
pub mod parsers;
pub mod year_2022;
pub mod year_2023;

pub mod prelude {
    pub use anyhow::{anyhow, bail, ensure, Context, Error, Result};
    pub use derive_deref::{Deref, DerefMut};
    pub use log::{debug, error, info, trace, warn};
    pub use thiserror::Error;

    pub use std::str::FromStr;
}
