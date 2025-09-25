use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Type alias for a shared, mutable reference to a node
type NodeRef<T> = Rc<RefCell<PartitionNode<T>>>;

/// A node in a doubly linked list representing a partition of data
#[derive(Debug)]
pub struct PartitionNode<T> {
    pub value: T,
    pub prev: Option<Weak<RefCell<PartitionNode<T>>>>,
    pub next: Option<NodeRef<T>>,
}

impl<T> PartitionNode<T> {
    /// Create a new node with no links
    pub fn new(value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(PartitionNode {
            value,
            prev: None,
            next: None,
        }))
    }

    /// Append a new node after the given node
    pub fn append(node: &NodeRef<T>, value: T) -> NodeRef<T> {
        let new_node = PartitionNode::new(value);
        {
            let mut node_borrow = node.borrow_mut();
            new_node.borrow_mut().prev = Some(Rc::downgrade(node));
            node_borrow.next = Some(Rc::clone(&new_node));
        }
        new_node
    }
}

// ===== Tests =====
#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_node_creation() {
        let node = PartitionNode::new(42);
        let node_borrow = node.borrow();
        assert_eq!(node_borrow.value, 42);
        assert!(node_borrow.prev.is_none());
        assert!(node_borrow.next.is_none());
    }

    #[test]
    fn test_append_node() {
        let head = PartitionNode::new(1);
        let second = PartitionNode::append(&head, 2);

        // Check forward link
        assert!(head.borrow().next.is_some());
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().value, 2);

        // Check backward link
        let prev = second.borrow().prev.as_ref().unwrap().upgrade().unwrap();
        assert_eq!(prev.borrow().value, 1);
    }

    #[test]
    fn test_multiple_appends() {
        let head = PartitionNode::new(10);
        let second = PartitionNode::append(&head, 20);
        let third = PartitionNode::append(&second, 30);

        // Forward traversal
        let mut current = Some(head.clone());
        let mut values = Vec::new();
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().next.clone();
        }
        assert_eq!(values, vec![10, 20, 30]);

        // Backward traversal
        let mut current = Some(third.clone());
        let mut backward_values = Vec::new();
        while let Some(node) = current {
            backward_values.push(node.borrow().value);
            current = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
        }
        assert_eq!(backward_values, vec![30, 20, 10]);
    }

    #[test]
    fn test_no_memory_leak() {
        let head = PartitionNode::new(1);
        let second = PartitionNode::append(&head, 2);
        let third = PartitionNode::append(&second, 3);

        // Drop all references to head
        drop(head);
        drop(second);

        // The last node should still be alive
        assert_eq!(Rc::strong_count(&third), 1);
        // Its prev still points to second, but as Weak, so it doesn't prevent drop
        assert!(third.borrow().prev.as_ref().unwrap().upgrade().is_none());
    }
}
