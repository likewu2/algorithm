use crate::tree::binary::node::Node;
use std::ptr::NonNull;

pub struct Tree<K, V> {
    pub root: Option<NonNull<Node<K, V>>>,
}

impl<K, V> Tree<K, V> {
    pub fn height(&self) -> usize {
        height(self.root)
    }
}

impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Tree { root: None }
    }
}

impl<K, V> Drop for Tree<K, V> {
    fn drop(&mut self) {
        fn visitor<K, V>(p: Option<NonNull<Node<K, V>>>) {
            if let Some(p) = p {
                let p = unsafe { Box::from_raw(p.as_ptr()) };
                visitor(p.left);
                visitor(p.right);
            }
        }
        visitor(self.root);
    }
}

fn height<K, V>(node: Option<NonNull<Node<K, V>>>) -> usize {
    node.map_or(0, |node| unsafe {
        let lh = height(node.as_ref().left);
        let rh = height(node.as_ref().right);
        1 + std::cmp::max(lh, rh)
    })
}
