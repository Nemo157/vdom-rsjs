use std::collections::HashMap;
use std::iter::IntoIterator;
use std::borrow::Cow;

use {VNode, VTag, VProperty};

impl<A> VTag<A> {
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
}

impl<A> VTag<A> {
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        VTag {
            name: tag.into().into_owned(),
            properties: HashMap::new(),
            children: Vec::new(),
            key: None,
            namespace: None,
        }
    }
}
