mod core;
#[cfg(feature = "macroquad")]
mod macroquad;

#[cfg(feature = "use_json")]
mod json;
use core::*;
pub mod prelude;
