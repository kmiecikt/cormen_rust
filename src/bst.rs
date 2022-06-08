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

impl<T: Copy + PartialOrd> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
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
}