type NodePointer<T> = Option<Box<Node<T>>>;

pub struct Node<T: Copy + PartialOrd> {
    pub value: T,
    left: NodePointer<T>,
    right: NodePointer<T>
}

impl<T: Copy + PartialOrd> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value: value, left: None, right: None }
    }
}

pub struct Tree<T: Copy + PartialOrd> {
    pub root: NodePointer<T>
}

// Finds a node with maximum value in a tree with the given root.
fn max_node_mut<T: Copy + PartialOrd>(root: &mut NodePointer<T>) -> &mut NodePointer<T> {
    let mut current = root;

    while current.is_some() && current.as_ref().unwrap().right.is_some() {
        let node = current.as_mut().unwrap();
        current = &mut node.right;
    }

    current
}

/// Deletes a given node. It must be a pointer to non-None node.
fn delete_node<T: Copy + PartialOrd>(root: &mut NodePointer<T>) {
    let mut this = root.take().unwrap();
    let left = this.left.take();
    let right = this.right.take();

    if left.is_none() && right.is_none() {
        return;
    }
    else if left.is_none() {
        root.replace(right.unwrap());
    }
    else if right.is_none() {
        root.replace(left.unwrap());
    }
    else {
        this.left = left;
        this.right = right;
        let next = max_node_mut(&mut this.left);
        this.value = next.as_ref().unwrap().value;
        delete_node(next);
        root.replace(this);
    }
}

/// The Drop trail for a tree. Because this data structure is recursive, the default, compiler-generated
/// implementation may cause stack overflows. 
/// This great link contains more details: https://rust-unofficial.github.io/too-many-lists/first-drop.html
impl<T: Copy + PartialOrd> Drop for Tree<T> {
    fn drop(&mut self) {
        if let Some(root_node) = self.root.take() {
            let mut stack = Vec::new();
            stack.push(root_node);
            
            while let Some(mut current_node) = stack.pop() {
                if let Some(left_node) = current_node.left.take() {
                    stack.push(left_node);
                }
                if let Some(right_node) = current_node.right.take() {
                    stack.push(right_node);
                }
            }
        }
    }
}

/// Implementation for the Binary Search Tree.
impl<T: Copy + PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }
    
    pub fn contains(self: &Tree<T>, value: T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            else if value < node.value {
                current = &node.left;
            }
            else {
                current = &node.right;
            }
        }
        
        false
    }
    
    pub fn insert(self: &mut Tree<T>, value: T) {
        let mut current = &mut self.root;
        
        while let Some(node) = current {
            if value <= node.value {
                current = &mut node.left;
            }
            else {
                current = &mut node.right;
            }
        }

        let new_node = Box::new(Node::new(value));
        current.replace(new_node);
    }

    pub fn find(self: &Tree<T>, value: T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            else if value < node.value {
                current = &node.left;
            }
            else {
                current = &node.right;
            }
        }

        false
    }

    pub fn remove(self: &mut Tree<T>, value: T) -> bool {
        let mut current = &mut self.root;

        while current.is_some() {
            let current_value  = current.as_ref().unwrap().value;
            if value == current_value {
                delete_node(current);
                return true;
            }
            else {
                if value < current_value {
                    current = &mut current.as_mut().unwrap().left;
                }
                else {
                    current = &mut current.as_mut().unwrap().right;
                }
            }
        }

        false
    }

    /// Finds the minimum value in the tree.
    /// # Examples
    /// ```
    /// use cormen_rust::bst::Tree; 
    /// let mut tree = Tree::new();
    /// assert_eq!(None, tree.min());
    /// 
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(0);
    /// assert_eq!(Some(0), tree.min());
    /// ```
    pub fn min(self: &Tree<T>) -> Option<T> {
        let mut current = &self.root;

        while let Some(node) = current {
            match &node.left {
                Some(_) => {
                    current = &node.left;
                },
                None => {
                    return Some(node.value);
                }
            }
        }

        None
    }

    /// Finds the maximum value in the tree.
    /// # Examples
    /// ```
    /// use cormen_rust::bst::Tree; 
    /// let mut tree = Tree::new();
    /// assert_eq!(None, tree.max());
    /// 
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(0);
    /// assert_eq!(Some(2), tree.max());
    /// ```
    pub fn max(self: &Tree<T>) -> Option<T> {
        let mut current = &self.root;
        
        while let Some(node) = current {
            match &node.right {
                Some(_) => {
                    current = &node.right;
                },
                None => {
                    return Some(node.value);
                }
            }
        }

        None
    }
    
    pub fn iter<'a>(self: &'a Tree<T>) -> TreeIntoIterator<'a, T> {
        let mut stack = Vec::new();
        
        if let Some(node) = &self.root {
            stack.push((false, node));
        }
        
        TreeIntoIterator { stack: stack }
    }
}

pub struct TreeIntoIterator<'a, T: Copy + PartialOrd> {
    stack: Vec<(bool, &'a Box<Node<T>>)>
}

impl<'a, T: Copy + PartialOrd> Iterator for TreeIntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        while let Some((left_visited, node)) = self.stack.pop() {
            if left_visited {
                if let Some(right) = &node.right {
                    self.stack.push((false, &right));
                }
                result = Some(node.value);
                break;
            }
            else {
                self.stack.push((true, node));
                if let Some(left) = &node.left {
                    self.stack.push((false, &left));
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert() {
        let input = vec![1, 4, 2, 8, 10, 3, 1];
        let mut expected = input.clone();
        expected.sort();

        let mut tree = Tree::new();
        for item in input {
            tree.insert(item);
        }
        
        let actual: Vec<i32> = tree.iter().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(0);
        tree.insert(4);

        assert!(tree.find(0));
        assert!(tree.find(2));
        assert!(tree.find(4));
        assert!(!tree.find(-1));
        assert!(!tree.find(1));
        assert!(!tree.find(3));
        assert!(!tree.find(5));
    }

    #[test]
    fn test_remove_leaf() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(0);

        assert!(tree.remove(0));
        assert!(!tree.find(0));

        assert!(!tree.remove(0));

        assert!(tree.find(1));
    }

    #[test]
    fn test_remove_parent_with_left_child() {
        let mut tree = Tree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(0);

        assert!(tree.remove(2));
        assert!(!tree.find(2));
        assert!(!tree.remove(2));

        assert!(tree.find(0));
        assert!(tree.find(1));
    }

    #[test]
    fn test_remove_parent_with_right_child() {
        let mut tree = Tree::new();
        tree.insert(0);
        tree.insert(1);
        tree.insert(2);

        assert!(tree.remove(0));
        assert!(!tree.find(0));
        assert!(!tree.remove(0));

        assert!(tree.find(1));
        assert!(tree.find(2));
    }

    #[test]
    fn test_remove_root_with_two_children() {
        let mut tree = Tree::new();
        tree.insert(0);  // root
        tree.insert(-2); // left branch
        tree.insert(-1);
        tree.insert(-3);
        tree.insert(2);  // right branch
        tree.insert(1);
        tree.insert(3);

        assert!(tree.remove(0));
        assert!(!tree.find(0));
        assert!(!tree.remove(0));

        for i in vec![-3, -1, -2, 1, 2, 3] {
            assert!(tree.find(i));
        }
    }
}