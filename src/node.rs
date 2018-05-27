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
