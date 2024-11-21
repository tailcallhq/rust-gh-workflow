extern crate proc_macro;

mod cargo;
pub mod error;
mod event;
mod expr;
pub mod generate;
pub mod release_plz;
mod rust_flag;
pub mod toolchain;
pub(crate) mod workflow;

pub use cargo::*;
pub use event::*;
pub use expr::*;
pub use rust_flag::*;
pub use workflow::*;

pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

mod private {
    pub trait Sealed {}
}
