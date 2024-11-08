pub mod error;
mod event;
mod generate;
mod rust_flag;
mod toolchain;
pub(crate) mod workflow;

pub use event::*;
pub use rust_flag::*;
pub use toolchain::*;
pub use workflow::*;

pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
