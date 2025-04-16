/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//I AM DONE
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }
    fn insert_by_node(root: &mut Box<TreeNode<T>>, value: T) {
        if value == root.value {
            return;
        } else if value < root.value {
            match root.left {
                None => root.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut l_child_r) => Self::insert_by_node(l_child_r, value),
            }
        } else {
            match root.right {
                None => root.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut r_child_r) => Self::insert_by_node(r_child_r, value),
            }
        }
    }
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut r) => Self::insert_by_node(r, value),
        }
    }
    fn serach_by_node(root: &Box<TreeNode<T>>, value: T) -> bool {
        if value == root.value {
            true
        } else if value < root.value {
            match root.left {
                None => false,
                Some(ref r) => Self::serach_by_node(r, value),
            }
        } else {
            match root.right {
                None => false,
                Some(ref r) => Self::serach_by_node(r, value),
            }
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            None => false,
            Some(ref r) => Self::serach_by_node(r, value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
