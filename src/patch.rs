use std::borrow::Cow;
use std::collections::HashMap;

use serde::ser::{Serialize, Serializer, SerializeMap};

use ::{ VNode, VTag, VProperty };

#[derive(Serialize)]
pub struct Remove {
    pub from: usize,
    pub key: Cow<'static, str>,
}

#[derive(Serialize)]
pub struct Insert {
    pub to: usize,
    pub key: Cow<'static, str>,
}

#[derive(Serialize)]
pub struct Reorder {
    pub removes: Vec<Remove>,
    pub inserts: Vec<Insert>,
}

#[derive(Serialize)]
pub enum VPropPatch<A> {
    Clear,
    Action(A),
    Text(Cow<'static, str>),
    Object(HashMap<Cow<'static, str>, Option<Cow<'static, str>>>),
}

#[derive(Serialize)]
pub enum VPatch<A> {
    None,
    Text(Cow<'static, str>),
    Node(VTag<A>),
    Props(HashMap<Cow<'static, str>, VPropPatch<A>>),
    Reorder(Reorder),
    Insert(VNode<A>),
    Remove(VNode<A>),
}

/// A minimal representation of the VNode tree for the patch algorithm to match
/// indices up with the real DOM tree
#[derive(Serialize)]
pub struct VPatchNode {
    pub children: Vec<VPatchNode>,
}

pub struct VPatches<A> {
    pub root: VPatchNode,
    pub patches: Vec<VPatch<A>>,
}

impl<'a, A> From<&'a VNode<A>> for VPatchNode {
    fn from(node: &'a VNode<A>) -> VPatchNode {
        VPatchNode {
            children: if let VNode::Tag(VTag { children, .. }) = node {
                children.iter().map(|n| VPatchNode::from(&*n)).collect()
            } else {
                Vec::new()
            }
        }
    }
}

impl<'a, A: Clone> From<&'a VProperty<A>> for VPropPatch<A> {
    fn from(prop: &'a VProperty<A>) -> VPropPatch<A> {
        match prop {
            VProperty::Action(action) => {
                VPropPatch::Action(action.clone())
            }
            VProperty::Text(text) => {
                VPropPatch::Text(text.clone())
            }
            VProperty::Object(vals) => {
                let vals = vals.iter()
                    .map(|(k, v)| (k.clone(), Some(v.clone())))
                    .collect();
                VPropPatch::Object(vals)
            }
        }
    }
}

impl<A: Serialize> Serialize for VPatches<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(self.patches.len() + 1))?;
        map.serialize_entry("a", &self.root)?;
        for (i, patch) in self.patches.iter().enumerate() {
            map.serialize_entry(&i.to_string(), patch)?;
        }
        map.end()
    }
}

