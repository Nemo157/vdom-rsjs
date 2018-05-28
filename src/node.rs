use std::collections::HashMap;

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
    pub children: Vec<VNode<A>>,
    pub key: Option<String>,
    pub namespace: Option<String>,
}

impl<A> VProperty<A> {
    pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VProperty<B> {
        match self {
            VProperty::Action(a) => VProperty::Action(f(a)),
            VProperty::Text(t) => VProperty::Text(t),
            VProperty::Object(o) => VProperty::Object(o),
        }
    }
}

impl<A> VNode<A> {
    pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VNode<B> {
        match self {
            VNode::Tag(t) => VNode::Tag(t.map_action(f)),
            VNode::Text(t) => VNode::Text(t),
        }
    }
}

impl<A> VTag<A> {
    pub fn map_action<B>(self, f: &impl Fn(A) -> B) -> VTag<B> {
        let VTag { name, properties, children, key, namespace } = self;
        let properties = properties
            .into_iter()
            .map(|(k, v)| (k, v.map_action(f)))
            .collect();
        let children = children
            .into_iter()
            .map(|c| c.map_action(f))
            .collect();
        VTag { name, properties, children, key, namespace }
    }
}
