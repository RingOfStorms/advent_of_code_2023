#[cfg(all(feature = "part1", feature = "part2"))]
compile_error!("Part 1 and Part 2 are mutually exclusive and cannot be enabled together");
pub mod utils;

pub mod prelude {
    pub use super::utils::{
        self,
        common::{BoxE, Result, SError, SResult},
        config::get_config,
    };
}
