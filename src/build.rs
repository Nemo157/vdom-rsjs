use std::collections::HashMap;
use std::iter::IntoIterator;
use std::borrow::Cow;

use {VNode, VTag, VProperty};

pub struct TagBuilder<A> {
    name: String,
    properties: HashMap<String, VProperty<A>>,
    children: Vec<VNode<A>>,
    key: Option<String>,
    namespace: Option<String>,
}

impl<A> TagBuilder<A> {
    pub fn prop(mut self, prop: impl Into<Cow<'static, str>>, value: impl Into<VProperty<A>>) -> Self {
        self.properties.insert(prop.into().into_owned(), value.into());
        self
    }

    pub fn props(mut self, props: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<VProperty<A>>)>) -> Self {
        self.properties.extend(props.into_iter().map(|(s, p)| (s.into().into_owned(), p.into())));
        self
    }

    pub fn child(mut self, child: impl Into<VNode<A>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<VNode<A>>>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into()));
        self
    }

    pub fn build(self) -> VNode<A> {
        VNode::Tag(VTag {
            name: self.name,
            properties: self.properties,
            children: self.children,
            key: self.key,
            namespace: self.namespace,
        })
    }
}

pub fn tag<A>(tag: impl Into<Cow<'static, str>>) -> TagBuilder<A> {
    TagBuilder {
        name: tag.into().into_owned(),
        properties: HashMap::new(),
        children: Vec::new(),
        key: None,
        namespace: None,
    }
}

impl<A> From<TagBuilder<A>> for VNode<A> {
    fn from(builder: TagBuilder<A>) -> Self {
        builder.build()
    }
}
