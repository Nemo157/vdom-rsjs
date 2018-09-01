use std::sync::Arc;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

use im::Vector;

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Deserialize, Clone)]
pub enum VProperty<A> {
    Action(A),
    Text(Cow<'static, str>),
    Object(HashMap<Cow<'static, str>, Cow<'static, str>>),
}

#[serde(rename_all = "lowercase")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VNode<A> {
    Tag(VTag<A>),
    Text(Cow<'static, str>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VTag<A> {
    pub name: Cow<'static, str>,
    pub properties: HashMap<Cow<'static, str>, VProperty<A>>,
    pub children: Vector<VNode<A>>,
    pub key: Option<Cow<'static, str>>,
    pub namespace: Option<Cow<'static, str>>,
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
        Arc::new(VNode::Text(self.into()))
    }
}

impl<A> IntoSharedVNode<A> for String {
    fn into_vnode(self) -> Arc<VNode<A>> {
        Arc::new(VNode::Text(self.into()))
    }
}

impl<A> From<VTag<A>> for VNode<A> {
    fn from(tag: VTag<A>) -> Self {
        VNode::Tag(tag)
    }
}

impl<A, S: Into<Cow<'static, str>>> From<S> for VNode<A> {
    fn from(s: S) -> Self {
        VNode::Text(s.into())
    }
}

impl<A, S: Into<Cow<'static, str>>> From<S> for VProperty<A> {
    fn from(s: S) -> Self {
        VProperty::Text(s.into())
    }
}

impl<A> fmt::Debug for VProperty<A> {
    #[cfg(feature = "nightly")]
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VProperty::Action(_) => f.debug_tuple("Action").field(&"<non-debug>").finish(),
            VProperty::Text(text) => f.debug_tuple("Text").field(text).finish(),
            VProperty::Object(object) => f.debug_tuple("Object").field(object).finish(),
        }
    }

    #[cfg(not(feature = "nightly"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VProperty::Action(_) => f.debug_tuple("Action").field(&"<non-debug>").finish(),
            VProperty::Text(text) => f.debug_tuple("Text").field(text).finish(),
            VProperty::Object(object) => f.debug_tuple("Object").field(object).finish(),
        }
    }
}

#[cfg(feature = "nightly")]
impl<A: fmt::Debug> fmt::Debug for VProperty<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VProperty::Action(action) => f.debug_tuple("Action").field(action).finish(),
            VProperty::Text(text) => f.debug_tuple("Text").field(text).finish(),
            VProperty::Object(object) => f.debug_tuple("Object").field(object).finish(),
        }
    }
}
