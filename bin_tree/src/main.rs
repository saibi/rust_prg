use std::cmp::Ordering;

/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord> Node<T> {
    pub fn new(v: T) -> Self {
        Self {
            value: v,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, v: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(v))),
            Some(node) => match v.cmp(&node.value) {
                Ordering::Less => node.left.insert(v),
                Ordering::Equal => {}
                Ordering::Greater => node.right.insert(v),
            },
        }
    }

    fn has(&self, v: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match v.cmp(&node.value) {
                Ordering::Less => node.left.has(v),
                Ordering::Equal => true,
                Ordering::Greater => node.right.has(v),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(node) => 1 + node.left.len() + node.right.len(),
        }
    }
}
/// 바이너리 트리를 사용하여 값 집합을 저장하는 컨테이너입니다.
///
/// 동일한 값이 여러 번 추가되면 한 번만 저장됩니다.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.
impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    fn insert(&mut self, v: T) {
        self.root.insert(v)
    }

    fn has(&self, v: &T) -> bool {
        self.root.has(v)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // 고유 항목이 아닙니다.
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
