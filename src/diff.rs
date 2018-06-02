use std::sync::Arc;
use std::borrow::Cow;
use std::collections::HashMap;

use im::Vector;

use ::{VNode, VTag, VPatch, VPatchNode, VPatches, VProperties, VProperty, VPropPatch};

fn diff_prop<A: Clone>(prev: &VProperty<A>, next: &VProperty<A>) -> Option<VPropPatch<A>> {
    if let (VProperty::Object(prev), VProperty::Object(next)) = (prev, next) {
        let mut values = HashMap::new();

        for (key, next_value) in next.iter() {
            match prev.get(key) {
                Some(prev_value) => {
                    if prev_value != next_value {
                        values.insert(key.clone(), Some(next_value.clone()));
                    }
                }
                None => {
                    values.insert(key.clone(), Some(next_value.clone()));
                }
            }
        }

        for key in prev.keys() {
            if next.get(key).is_none() {
                values.insert(key.clone(), None);
            }
        }

        if values.is_empty() {
            None
        } else {
            Some(VPropPatch::Object(values))
        }
    } else {
        Some(next.into())
    }
}

fn diff_props<A: Clone>(prev: &VProperties<A>, next: &VProperties<A>) -> HashMap<Cow<'static, str>, VPropPatch<A>> {
    let mut props = HashMap::new();

    for (key, next_value) in next.iter() {
        if let Some(prev_value) = prev.get(key) {
            if let Some(patch) = diff_prop(prev_value, next_value) {
                props.insert(key.clone(), patch);
            }
        } else {
            props.insert(key.clone(), next_value.into());
        }
    }

    props
}

fn diff_children<A: Clone>(patches: &mut Vec<VPatch<A>>, prev: &Vector<VNode<A>>, next: &Vector<VNode<A>>) {
    // TODO: Really complicated re-ordering thingy needed...
}

fn diff_tags<A: Clone>(patches: &mut Vec<VPatch<A>>, prev: &VTag<A>, next: &VTag<A>) {
    let props = diff_props(&prev.properties, &next.properties);
    if !props.is_empty() {
        patches.push(VPatch::Props(props));
    }
    diff_children(patches, &prev.children, &next.children);
}

fn diff_nodes<A: Clone>(patches: &mut Vec<VPatch<A>>, prev: Option<&VNode<A>>, next: Option<&VNode<A>>) {
    let next = if let Some(next) = next {
        next
    } else {
        if let Some(prev) = prev {
            patches.push(VPatch::Remove(prev.clone()));
        }
        return
    };

    match next {
        VNode::Tag(ref next) => {
            if let Some(VNode::Tag(ref prev)) = prev {
                if prev.is_same_tag(next) {
                    diff_tags(patches, prev, next);
                } else {
                    patches.push(VPatch::Node(next.clone()));
                }
            } else {
                patches.push(VPatch::Node(next.clone()));
            }
        }
        VNode::Text(ref next) => {
            if let Some(VNode::Text(ref prev)) = prev {
                if prev != next {
                    patches.push(VPatch::Text(next.clone()));
                }
            } else {
                patches.push(VPatch::Text(next.clone()));
            }
        }
    }
}

pub fn diff<A: Clone>(prev: Arc<VNode<A>>, next: Arc<VNode<A>>) -> VPatches<A> {
    let mut patches = VPatches {
        root: VPatchNode::from(&*next),
        patches: Vec::new(),
    };
    diff_nodes(&mut patches.patches, Some(&*prev), Some(&*next));
    patches
}
