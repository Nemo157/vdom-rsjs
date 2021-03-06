#![cfg_attr(feature = "nightly", feature(specialization))]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate im;

mod node;
pub mod patch;
mod diff;
mod build;
pub mod render;

pub use node::{VNode, VTag, VProperty};
pub use patch::{VPatch};

pub use diff::diff;
