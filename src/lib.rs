mod node;
pub mod patch;
mod diff;
mod build;
pub mod render;

pub use crate::node::{VNode, VTag, VProperty};
pub use crate::patch::{VPatch};

pub use crate::diff::diff;
