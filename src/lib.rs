pub mod utils;

pub mod prelude {
    pub use super::utils::{
        self,
        common::{
            BoxE,
            Result,
            SError,
            SResult,
        },
        config::get_config,
    };
}
