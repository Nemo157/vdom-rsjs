extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate im;

mod node;
pub mod patch;
mod diff;
mod build;
pub mod render;

pub use node::{VNode, VTag, VProperty, VProperties};
pub use patch::{VPatch, VPatchNode, VPatches, VPropPatch};

pub use diff::diff;
