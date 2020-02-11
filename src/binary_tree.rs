use ego_tree::{NodeMut, NodeRef, Tree};

/// Wrapper around a ego_tree::Tree that constrains functionality / API
/// to a binary tree. Always contains at least one node.
pub struct BinaryTree<T> {
    inner: Tree<Option<T>>,
}

impl<T> BinaryTree<T> {
    pub fn new(root_value: T) -> Self {
        let mut tree = Tree::new(Some(root_value));
        let mut root = tree.root_mut();
        root.append(None);
        root.append(None);
        Self { inner: tree }
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> BinaryNodeRef<T> {
        BinaryNodeRef::wrap(self.inner.root())
    }

    /// Returns a mutator of the root node.
    pub fn root_mut(&mut self) -> BinaryNodeMut<T> {
        BinaryNodeMut::wrap(self.inner.root_mut())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BinaryNodeRef<'a, T> {
    inner: NodeRef<'a, Option<T>>,
}

impl<'a, T> BinaryNodeRef<'a, T> {
    /// Return the left child, if exists.
    pub fn left(&self) -> Option<BinaryNodeRef<'a, T>> {
        let left = self.inner.children().next().expect("always has children");
        if left.value().is_none() {
            return None;
        }

        Some(BinaryNodeRef::wrap(left))
    }

    /// Return the right child, if exists.
    pub fn right(&self) -> Option<BinaryNodeRef<'a, T>> {
        let mut children = self.inner.children();
        let _left = children.next().expect("always has children");

        let right = children.next().expect("always has children");
        if right.value().is_none() {
            return None;
        }

        Some(BinaryNodeRef::wrap(right))
    }

    /// Get the value for this node.
    pub fn value(&self) -> &T {
        self.inner.value().as_ref().expect("exists")
    }

    fn wrap(node: NodeRef<'a, Option<T>>) -> Self {
        Self { inner: node }
    }
}

#[derive(Debug)]
pub struct BinaryNodeMut<'a, T> {
    inner: NodeMut<'a, Option<T>>,
}

impl<'a, T> BinaryNodeMut<'a, T> {
    fn left_inner(&mut self) -> NodeMut<Option<T>> {
        self.inner.first_child().expect("exists")
    }

    fn right_inner(&mut self) -> NodeMut<Option<T>> {
        self.inner.last_child().expect("exists")
    }

    /// Return the left child, if exists.
    pub fn left(&mut self) -> Option<BinaryNodeMut<T>> {
        let mut left_inner = self.left_inner();
        if left_inner.value().is_none() {
            return None;
        }

        Some(BinaryNodeMut::wrap(left_inner))
    }

    /// Return the right child, if exists.
    pub fn right(&mut self) -> Option<BinaryNodeMut<T>> {
        let mut right_inner = self.right_inner();
        if right_inner.value().is_none() {
            return None;
        }

        Some(BinaryNodeMut::wrap(right_inner))
    }

    /// Get the value for this node.
    pub fn value(&mut self) -> &mut T {
        self.inner.value().as_mut().expect("exists")
    }

    /// Set the right child to value and return the node.
    pub fn set_right(&mut self, value: T) -> BinaryNodeMut<T> {
        let mut right_inner = self.right_inner();
        *right_inner.value() = Some(value);
        // Create left / right nodes for new node if not present.
        if !right_inner.has_children() {
            right_inner.append(None);
            right_inner.append(None);
        }
        BinaryNodeMut::wrap(right_inner)
    }

    /// Set the left child to value and return the node.
    pub fn set_left(&mut self, value: T) -> BinaryNodeMut<T> {
        let mut left_inner = self.left_inner();
        *left_inner.value() = Some(value);
        // Create left / right nodes for new node if not present.
        if !left_inner.has_children() {
            left_inner.append(None);
            left_inner.append(None);
        }
        BinaryNodeMut::wrap(left_inner)
    }

    fn wrap(node: NodeMut<'a, Option<T>>) -> Self {
        Self { inner: node }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree_api_works() {
        let mut tree = BinaryTree::new(5);
        assert!(tree.root_mut().left().is_none());
        assert!(tree.root_mut().right().is_none());

        let mut root = tree.root_mut();
        let mut left = root.set_left(3);
        assert_eq!(left.value(), &3);
        assert!(left.left().is_none());
        assert!(left.right().is_none());
    }
}
