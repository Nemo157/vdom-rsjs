use std::sync::Arc;
use std::collections::HashMap;

use crate::VNode;

struct CacheValue<A> {
    used: bool,
    new: bool,
    node: Arc<VNode<A>>,
}

pub trait Cache<A> {
    fn render(&mut self, item: Arc<dyn Render<A>>) -> Arc<VNode<A>>;
}

pub trait Render<A> {
    fn render(&self, cache: &mut dyn Cache<A>) -> VNode<A>;
}

pub struct NoCache {
    _private: (),
}

pub struct TopCache<A> {
    cache: HashMap<*const dyn Render<A>, CacheValue<A>>,
}

pub struct ChildCache<'a, A> {
    top: &'a mut TopCache<A>,
}

impl<A> TopCache<A> {
    pub fn new() -> TopCache<A> {
        TopCache {
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, item: Arc<dyn Render<A>>) -> (Option<Arc<VNode<A>>>, Arc<dyn Render<A>>) {
        let ptr = Arc::into_raw(item);
        let node = self.cache.get_mut(&ptr).map(CacheValue::node);
        // Safe because we only lent the pointer out to a function without a
        // `Clone` bound.
        let item = unsafe { Arc::from_raw(ptr) };
        (node, item)
    }
}

impl NoCache {
    pub fn new() -> NoCache {
        NoCache {
            _private: ()
        }
    }
}

impl<A> Cache<A> for TopCache<A> {
    fn render(&mut self, item: Arc<dyn Render<A>>) -> Arc<VNode<A>> {
        for value in self.cache.values_mut() {
            value.used = false;
            value.new = false;
        }

        let (node, item) = self.get(item);
        let node = node.unwrap_or_else(|| {
            let node = Arc::new(item.render(&mut ChildCache { top: self }));
            self.cache.insert(Arc::into_raw(item), CacheValue::new(node.clone()));
            node
        });

        println!(
            "Cache used {}/{} entries + {} new entries",
            self.cache.values().filter(|e| e.used && !e.new).count(),
            self.cache.values().filter(|e| !e.new).count(),
            self.cache.values().filter(|e| e.new).count());

        self.cache.retain(|ptr, value| {
            if !value.used {
                // Safe? because the pointer will only be temporarily dangling
                // until the entry is removed during this retain call.
                unsafe { Arc::from_raw(*ptr) };
            }
            value.used
        });

        node
    }
}

impl<'a, A> Cache<A> for ChildCache<'a, A> {
    fn render(&mut self, item: Arc<dyn Render<A>>) -> Arc<VNode<A>> {
        let (node, item) = self.top.get(item);
        node.unwrap_or_else(|| {
            let node = Arc::new(item.render(self));
            self.top.cache.insert(Arc::into_raw(item), CacheValue::new(node.clone()));
            node
        })
    }
}

impl<A> Cache<A> for NoCache {
    fn render(&mut self, item: Arc<dyn Render<A>>) -> Arc<VNode<A>> {
        Arc::new(item.render(self))
    }
}

impl<A> CacheValue<A> {
    fn new(node: Arc<VNode<A>>) -> CacheValue<A> {
        CacheValue {
            used: true,
            new: true,
            node: node,
        }
    }

    fn node(&mut self) -> Arc<VNode<A>> {
        self.used = true;
        self.node.clone()
    }
}
