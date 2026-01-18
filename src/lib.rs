mod core;
#[cfg(feature = "macroquad")]
mod macroquad;

#[cfg(feature = "use_json")]
mod json;
pub mod prelude;
// mod struct_diffs;
// per uso nella libreria stessa
// (con i vari use super::*):
