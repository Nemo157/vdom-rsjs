use std::sync::Arc;
use std::borrow::Cow;
use std::collections::HashMap;

use im::Vector;

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VProperty<A> {
    Action(A),
    Text(String),
    Object(HashMap<String, String>),
}

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VNode<A> {
    Tag(VTag<A>),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VTag<A> {
    pub name: String,
    pub properties: HashMap<String, VProperty<A>>,
    pub children: Vector<VNode<A>>,
    pub key: Option<String>,
    pub namespace: Option<String>,
}

// impl<A> VProperty<A> {
//     pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VProperty<B> {
//         match self {
//             VProperty::Action(a) => VProperty::Action(f(a)),
//             VProperty::Text(t) => VProperty::Text(t),
//             VProperty::Object(o) => VProperty::Object(o),
//         }
//     }
// }
//
// impl<A> VNode<A> {
//     pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VNode<B> {
//         match self {
//             VNode::Tag(t) => VNode::Tag(t.map_action(f)),
//             VNode::Text(t) => VNode::Text(t),
//         }
//     }
// }
//
// impl<A> VTag<A> {
//     pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VTag<B> {
//         let VTag { name, properties, children, key, namespace } = self;
//         let properties = properties
//             .into_iter()
//             .map(|(k, v)| (k, v.map_action(f)))
//             .collect();
//         let children = children
//             .into_iter()
//             .map(|c| c.map_action(f))
//             .collect();
//         VTag { name, properties, children, key, namespace }
//     }
// }

pub trait IntoSharedVNode<A> {
    fn into_vnode(self) -> Arc<VNode<A>>;
}

impl<A> IntoSharedVNode<A> for Arc<VNode<A>> {
    fn into_vnode(self) -> Arc<VNode<A>> {
        self
    }
}

impl<A> IntoSharedVNode<A> for VNode<A> {
    fn into_vnode(self) -> Arc<VNode<A>> {
        Arc::new(self)
    }
}

impl<A> IntoSharedVNode<A> for VTag<A> {
    fn into_vnode(self) -> Arc<VNode<A>> {
        Arc::new(VNode::Tag(self))
    }
}

impl<A> IntoSharedVNode<A> for &'static str {
    fn into_vnode(self) -> Arc<VNode<A>> {
        Arc::new(VNode::Text(self.to_owned()))
    }
}

impl<A> IntoSharedVNode<A> for String {
    fn into_vnode(self) -> Arc<VNode<A>> {
        Arc::new(VNode::Text(self))
    }
}

impl<A> From<VTag<A>> for VNode<A> {
    fn from(tag: VTag<A>) -> Self {
        VNode::Tag(tag)
    }
}

impl<A, S: Into<Cow<'static, str>>> From<S> for VNode<A> {
    fn from(s: S) -> Self {
        VNode::Text(s.into().into_owned())
    }
}

impl<A, S: Into<Cow<'static, str>>> From<S> for VProperty<A> {
    fn from(s: S) -> Self {
        VProperty::Text(s.into().into_owned())
    }
}
