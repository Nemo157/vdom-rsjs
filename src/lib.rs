extern crate serde;
#[macro_use]
extern crate serde_derive;

mod node;
pub mod patch;
mod diff;
mod build;

pub use node::{VNode, VTag, VProperty};
pub use patch::{VPatch};

pub use diff::diff;
