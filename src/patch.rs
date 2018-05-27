use std::collections::HashMap;

use ::{ VNode, VTag, VProperty };

#[derive(Serialize, Deserialize)]
pub struct Remove {
    from: usize,
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Insert {
    to: usize,
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Reorder {
    removes: Vec<Remove>,
    inserts: Vec<Insert>,
}

#[derive(Serialize, Deserialize)]
pub enum VPatch<A> {
    None,
    Text(String),
    Node(VTag<A>),
    Props { prev: HashMap<String, VProperty<A>>, next: HashMap<String, VProperty<A>> },
    Reorder(Reorder),
    Insert(VNode<A>),
    Remove(VNode<A>),
}
