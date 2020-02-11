/// Creates a binary tree from expressions.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate ego_binary_tree;
/// # fn main() {
/// let tree = binary_tree!("root");
/// # }
/// ```
///
/// ```
/// #[macro_use] extern crate ego_binary_tree;
/// # fn main() {
/// let tree = binary_tree! {
///     "root" => {
///         left: "child a",
///         right: "child b" => {
///             right: "grandchild a",
///         },
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! binary_tree {
    (@ $n:ident { }) => { };

    // Node with only right node with children.
    (@ $n:ident { right: $value:expr => $children:tt$(,)* }) => {
        {
            let mut node = $n.set_right($value);
            binary_tree!(@ node $children)
        }
    };

    // Node with only left node with children.
    (@ $n:ident { left: $value:expr => $children:tt$(,)* }) => {
        {
            let mut node = $n.set_left($value);
            binary_tree!(@ node $children)
        }
    };

    // Node with only left leaf node.
    (@ $n:ident { left: $value:expr$(,)* }) => {
        { $n.set_left($value); }
    };

    // Node with only right leaf node.
    (@ $n:ident { right: $value:expr$(,)* }) => {
        { $n.set_right($value); }
    };

    // Node with left leaf and right with children.
    (@ $n:ident { left: $left_value:expr, right: $right_value:expr => $right_children:tt$(,)* }) => {
        {
            $n.set_left($left_value);
            let mut right_node = $n.set_right($right_value);
            binary_tree!(@ right_node $right_children);
        }
    };

    // Node with left and right leaf.
    (@ $n:ident { left: $left_value:expr, right: $right_value:expr$(,)* }) => {
        {
            $n.set_left($left_value);
            $n.set_right($right_value);
        }
    };

    // Node with left with children and right leaf.
    (@ $n:ident { left: $left_value:expr => $left_children:tt, right: $right_value:expr$(,)* }) => {
        {
            let mut left_node = $n.set_left($left_value);
            binary_tree!(@ left_node $left_children);
            $n.set_right($right_value);
        }
    };

    // Node with both left and right nodes.
    (@ $n:ident { left: $left_value:expr => $left_children:tt, right: $right_value:expr => $right_children:tt$(,)* }) => {
        {
            let mut left_node = $n.set_left($left_value);
            binary_tree!(@ left_node $left_children);
            let mut right_node = $n.set_right($right_value);
            binary_tree!(@ right_node $right_children);
        }
    };

    ($root:expr) => { $crate::BinaryTree::new($root) };

    ($root:expr => $children:tt) => {
        {
            let mut tree = $crate::BinaryTree::new($root);
            {
                let mut node = tree.root_mut();
                binary_tree!(@ node $children);
            }
            tree
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_root_works() {
        let tree = binary_tree!('a');
        assert_eq!(tree.root().value(), &'a');
    }

    #[test]
    fn complicated_tree_works() {
        let tree = binary_tree! {
            "root" => {
                left: "left",
                right: "right" => {
                    right: "rightright" => {
                        left: "rightrightleft"
                    },
                },
            }
        };
        assert_eq!(tree.root().value(), &"root");

        let left = tree.root().left().unwrap();
        assert_eq!(left.value(), &"left");
        assert!(left.left().is_none());
        assert!(left.right().is_none());

        let right = tree.root().right().unwrap();
        assert_eq!(right.value(), &"right");
        assert!(right.left().is_none());

        let rightright = right.right().unwrap();
        assert_eq!(rightright.value(), &"rightright");

        let rightrightleft = rightright.left().unwrap();
        assert_eq!(rightrightleft.value(), &"rightrightleft");
    }
}
