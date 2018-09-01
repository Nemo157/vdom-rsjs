use std::collections::HashMap;
use std::iter::IntoIterator;
use std::borrow::Cow;

use im::Vector;

use {VTag, VProperty, node::IntoSharedVNode};

impl<A> VTag<A> {
    pub fn attr(mut self, attr: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self {
        {
            let attrs = self.properties.entry("attributes".into()).or_insert(VProperty::Object(HashMap::new()));
            if let VProperty::Object(attrs) = attrs {
                attrs.insert(attr.into(), value.into());
            } else {
                panic!("Unexpected property type for attributes: {:?}", attrs);
            }
        }
        self
    }

    pub fn attrs(mut self, attrs: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>) -> Self {
        {
            let attributes = self.properties.entry("attributes".into()).or_insert(VProperty::Object(HashMap::new()));
            if let VProperty::Object(attributes) = attributes {
                attributes.extend(attrs.into_iter().map(|(a, v)| (a.into(), v.into())));
            } else {
                panic!("Unexpected property type for attributes: {:?}", attributes);
            }
        }
        self
    }

    pub fn prop(mut self, prop: impl Into<Cow<'static, str>>, value: impl Into<VProperty<A>>) -> Self {
        self.properties.insert(prop.into(), value.into());
        self
    }

    pub fn props(mut self, props: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<VProperty<A>>)>) -> Self {
        self.properties.extend(props.into_iter().map(|(s, p)| (s.into(), p.into())));
        self
    }

    pub fn child(mut self, child: impl IntoSharedVNode<A>) -> Self {
        self.children.push_back_mut(child.into_vnode());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoSharedVNode<A>>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_vnode()));
        self
    }
}

impl<A> VTag<A> {
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        VTag {
            name: tag.into(),
            properties: HashMap::new(),
            children: Vector::new(),
            key: None,
            namespace: None,
        }
    }
}
